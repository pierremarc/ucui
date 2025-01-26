use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
};
use serde::Serialize;
use shakmaty::fen::Fen;
use tokio::sync::{
    broadcast::{channel, Receiver, Sender},
    Mutex,
};

use crate::state::UcuiState;

#[derive(Clone)]
pub enum DBMessage {
    Version(u128),
}

#[derive(Clone)]
pub struct MonitorDB {
    version: u128,
    data: HashMap<String, Fen>,
}
impl MonitorDB {
    fn set(&mut self, key: String, game: Fen) {
        let _ = self.data.insert(key, game);
        self.version += 1;
    }

    fn del(&mut self, key: String) {
        let _ = self.data.remove(&key);
        self.version += 1;
    }

    fn entries(&self) -> Vec<(String, Fen)> {
        self.data
            .iter()
            .map(|(k, g)| (k.clone(), g.clone()))
            .collect()
    }
}

pub type SharedDB = Arc<Mutex<MonitorDB>>;

pub struct Monitor {
    tx: Sender<DBMessage>,
    rx: Receiver<DBMessage>,
    db: SharedDB,
}

impl Monitor {
    pub fn new() -> Self {
        let (tx, rx) = channel(100);
        Monitor {
            tx,
            rx,
            db: Arc::new(Mutex::new(MonitorDB {
                version: 0,
                data: HashMap::new(),
            })),
        }
    }

    pub async fn set(&self, key: String, game: Fen) {
        let mut db = self.db.lock().await;
        db.set(key, game);
        let _ = self.tx.send(DBMessage::Version(db.version));
    }
    pub async fn del(&self, key: String) {
        let mut db = self.db.lock().await;
        db.del(key);
        let _ = self.tx.send(DBMessage::Version(db.version));
    }

    pub async fn entries(&self) -> Vec<(String, Fen)> {
        let db = self.db.lock().await;
        db.entries()
    }

    pub async fn recv(&mut self) -> Option<DBMessage> {
        self.rx.recv().await.ok()
    }
}

impl Clone for Monitor {
    fn clone(&self) -> Self {
        Monitor {
            db: self.db.clone(),
            tx: self.tx.clone(),
            rx: self.tx.subscribe(),
        }
    }
}

#[derive(Clone, Serialize)]
#[serde(tag = "_tag")]
pub enum MonitorMessage {
    Init { games: Vec<(String, String)> },
    Update { games: Vec<(String, String)> },
}

impl MonitorMessage {
    fn init(games: Vec<(String, Fen)>) -> Message {
        Message::text(
            serde_json::to_string(&MonitorMessage::Init {
                games: games
                    .into_iter()
                    .map(|(id, fen)| (id, fen.to_string()))
                    .collect(),
            })
            .unwrap(),
        )
    }
    fn update(games: Vec<(String, Fen)>) -> Message {
        Message::text(
            serde_json::to_string(&MonitorMessage::Update {
                games: games
                    .into_iter()
                    .map(|(id, fen)| (id, fen.to_string()))
                    .collect(),
            })
            .unwrap(),
        )
    }
}

async fn handle_socket(mut socket: WebSocket, mut state: UcuiState) {
    let _ = socket
        .send(MonitorMessage::init(state.monitor.entries().await))
        .await;

    let mut last_seen = 0u128;
    loop {
        if let Some(DBMessage::Version(version)) = state.monitor.recv().await {
            if version > last_seen {
                last_seen = version;
                let _ = socket
                    .send(MonitorMessage::update(state.monitor.entries().await))
                    .await;
            }
        }
    }
}

pub async fn handler(ws: WebSocketUpgrade, State(server_state): State<UcuiState>) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, server_state))
}

use std::thread;

use copypasta::{ClipboardContext, ClipboardProvider};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use shakmaty::Position;

use super::{
    Screen, KEY_EXPORT_FEN, KEY_EXPORT_PGN, KEY_GO_HOME, KEY_GO_INFO, KEY_GO_LOGS, KEY_GO_PLAY,
    KEY_START_GAME,
};
use crate::config::get_engine_color;
use crate::export::{export_fen, export_pgn};
use crate::state::{State, Store};

fn clipboard_set<C: Into<String>>(content: C) {
    let content: String = content.into();
    log::info!("[clipboard_set] {}", &content);
    if let Ok(mut ctx) = ClipboardContext::new() {
        ctx.set_contents(content.clone()).expect("clipboard failed");
        ctx.set_contents(content).expect("clipboard failed");
    } else {
        log::warn!("!! failed to get a clipbard context")
    };
}

fn handle_move_input(store: &Store, state: &State, c: char) {
    if state.game().turn() == get_engine_color().other() {
        let base = match state.avail_input.clone() {
            None => format!("{c}"),
            Some(i) => format!("{i}{c}"),
        };

        store.update_avail_input(Some(base));
    }
}

fn handle_key_event_global(store: &Store, _state: &State, key_event: KeyEvent) -> bool {
    match key_event.code {
        KeyCode::Esc => {
            store.update_exit(true);
            false
        }
        KeyCode::Char(KEY_GO_HOME) => {
            store.update_screen(Screen::Home);
            false
        }
        KeyCode::Char(KEY_GO_INFO) => {
            store.update_screen(Screen::Info);
            false
        }
        KeyCode::Char(KEY_GO_PLAY) => {
            store.update_screen(Screen::Play);
            false
        }
        KeyCode::Char(KEY_GO_LOGS) => {
            store.update_screen(Screen::Log);
            false
        }

        _ => true,
    }
}

fn handle_key_event_on_home(store: &Store, state: &State, key_event: KeyEvent) {
    if handle_key_event_global(store, state, key_event) {
        if let KeyCode::Char(KEY_START_GAME) = key_event.code {
            store.update_game_started(true)
        }
    }
}
fn handle_key_event_on_info(store: &Store, state: &State, key_event: KeyEvent) {
    if handle_key_event_global(store, state, key_event) {
        log::info!("[handle_key_event_on_info] {}", key_event.code);
        if let KeyCode::Char(KEY_EXPORT_PGN) = key_event.code {
            clipboard_set(export_pgn(&state.game(), &state.hist));
        }
        if let KeyCode::Char(KEY_EXPORT_FEN) = key_event.code {
            clipboard_set(export_fen(&state.game()));
        }
    }
}
fn handle_key_event_on_play(store: &Store, state: &State, key_event: KeyEvent) {
    if handle_key_event_global(store, state, key_event) {
        match key_event.code {
            KeyCode::Char(c) => handle_move_input(store, state, c),
            KeyCode::Backspace => store.update_avail_input(None),
            KeyCode::Enter => store.update_validate_input(true),
            _ => {}
        }
    }
}
fn handle_key_event_on_log(store: &Store, state: &State, key_event: KeyEvent) {
    let _ = handle_key_event_global(store, state, key_event);
}

fn handle_key_event(store: &Store, key_event: KeyEvent) {
    if let Ok(state) = store.current_state() {
        match state.screen {
            Screen::Home => handle_key_event_on_home(store, &state, key_event),
            Screen::Info => handle_key_event_on_info(store, &state, key_event),
            Screen::Play => handle_key_event_on_play(store, &state, key_event),
            Screen::Log => handle_key_event_on_log(store, &state, key_event),
        }
    }
}

pub fn event_loop(store: Store) {
    thread::spawn(move || loop {
        if let Ok(ev) = event::read() {
            match ev {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    handle_key_event(&store, key_event);
                }
                _ => {}
            };
        }
    });
}

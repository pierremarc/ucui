use crossterm::event::{KeyCode, KeyEvent};
use shakmaty::Position;

use super::{
    Screen, KEY_EXPORT_FEN, KEY_EXPORT_PGN, KEY_GO_HOME, KEY_GO_INFO, KEY_GO_LOGS, KEY_GO_PLAY,
    KEY_START_GAME,
};
use crate::app::App;
use crate::config::get_engine_color;
use crate::export::{export_fen, export_pgn};

fn clipboard_set<C: Into<String>>(content: C) {
    use copypasta::{ClipboardContext, ClipboardProvider};
    if let Ok(mut ctx) = ClipboardContext::new() {
        ctx.set_contents(content.into()).expect("clipboard failed");
    };
}

fn handle_move_input(app: &mut App, c: char) {
    if app.game.turn() == get_engine_color().other() {
        let base = match app.input_move.clone() {
            None => format!("{c}"),
            Some(i) => format!("{i}{c}"),
        };

        app.input_move = Some(base);
    }
}

fn handle_key_event_global(app: &mut App, key_event: KeyEvent) -> bool {
    match key_event.code {
        KeyCode::Esc => {
            app.exit();
            false
        }
        KeyCode::Char(KEY_GO_HOME) => {
            app.screen = Screen::Home;
            false
        }
        KeyCode::Char(KEY_GO_INFO) => {
            app.screen = Screen::Info;
            false
        }
        KeyCode::Char(KEY_GO_PLAY) => {
            app.screen = Screen::Play;
            false
        }
        KeyCode::Char(KEY_GO_LOGS) => {
            app.screen = Screen::Log;
            false
        }

        _ => true,
    }
}

fn handle_key_event_on_home(app: &mut App, key_event: KeyEvent) {
    if handle_key_event_global(app, key_event) {
        if let KeyCode::Char(KEY_START_GAME) = key_event.code {
            app.start_game()
        }
    }
}
fn handle_key_event_on_info(app: &mut App, key_event: KeyEvent) {
    if handle_key_event_global(app, key_event) {
        if let KeyCode::Char(KEY_EXPORT_PGN) = key_event.code {
            clipboard_set(export_pgn(&app.game, &app.hist));
        }
        if let KeyCode::Char(KEY_EXPORT_FEN) = key_event.code {
            clipboard_set(export_fen(&app.game));
        }
    }
}
fn handle_key_event_on_play(app: &mut App, key_event: KeyEvent) {
    if handle_key_event_global(app, key_event) {
        match key_event.code {
            KeyCode::Char(c) => handle_move_input(app, c),
            KeyCode::Backspace => app.clear_input(),
            KeyCode::Enter => app.validate_move_input(),
            _ => {}
        }
    }
}
fn handle_key_event_on_log(app: &mut App, key_event: KeyEvent) {
    let _ = handle_key_event_global(app, key_event);
}

pub fn handle_key_event(app: &mut App, key_event: KeyEvent) {
    match app.screen {
        Screen::Home => handle_key_event_on_home(app, key_event),
        Screen::Info => handle_key_event_on_info(app, key_event),
        Screen::Play => handle_key_event_on_play(app, key_event),
        Screen::Log => handle_key_event_on_log(app, key_event),
    }
}

use ratatui::{
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Padding},
    Frame,
};

use super::Screen;

fn item<'a>(k: &'a str, name: &'a str) -> Span<'a> {
    Span::raw(format!(" {name} [{k}]"))
    // Span::raw(format!("<{k}> {name} "))
}

fn render_global(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Line::default().spans([
            item("Esc", "Exit"),
            item("1", "Home"),
            item("2", "Info"),
            item("3", "Game"),
            // item("4", "Logs"),
        ]),
        area,
    );
}

fn render_home(frame: &mut Frame, area: Rect) {
    frame.render_widget(Line::default().spans([item("Space", "Play")]), area);
}
fn render_info(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Line::default().spans([item("P", "Copy PGN"), item("F", "Copy FEN")]),
        area,
    );
}
fn render_play(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Line::default().spans([
            item("Up|Down", "Select Piece"),
            item("Left|Right", "Select Move"),
            item("Enter|Space", "Play Move"),
        ]),
        area,
    );
}

fn render_log(_frame: &mut Frame, _area: Rect) {}

pub fn render(screen: &Screen, frame: &mut Frame) -> Rect {
    let [area_main, area] =
        Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]).areas(frame.area());
    let [left, right] = Layout::horizontal(Constraint::from_percentages([30, 70])).areas(area);

    let pad = 0u16;
    let bl = Block::bordered()
        .title("Global")
        .padding(Padding::uniform(pad));
    let br = Block::bordered()
        .title(screen.name())
        .padding(Padding::uniform(pad));
    let left_inner = bl.inner(left);
    let right_inner = br.inner(right);
    frame.render_widget(bl, left);
    frame.render_widget(br, right);

    render_global(frame, left_inner);
    match screen {
        Screen::Home => render_home(frame, right_inner),
        Screen::Info => render_info(frame, right_inner),
        Screen::Play => render_play(frame, right_inner),
        Screen::Log => render_log(frame, right_inner),
    };
    area_main
}

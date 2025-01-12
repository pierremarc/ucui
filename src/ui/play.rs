use chrono::Timelike;
use ratatui::style::{Color as UiColor, Style};
use ratatui::widgets::Padding;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::Stylize,
    widgets::{Block, Paragraph},
    Frame,
};

use shakmaty::{Chess, Color, Move, Position};
use tui_big_text::{BigText, PixelSize};

use crate::config::get_engine_color;
use crate::util::{px_height, san_format_move};

use super::AppState;

static FONT_SIZE_CLOCK: PixelSize = PixelSize::Quadrant;
static FONT_SIZE_ENGINE_MOVE: PixelSize = PixelSize::Full;

fn render_empty_input(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Paragraph::new(
            "
        Engine to move...
        ",
        )
        .block(Block::bordered().title("input")),
        area,
    );
}

fn clock_style(c: Color, turn: Color) -> Style {
    if turn == c {
        Style::default().fg(UiColor::Black).bg(UiColor::Gray)
    } else {
        Style::default().fg(UiColor::DarkGray).bg(UiColor::Black)
    }
}

fn render_clock(clock: &crate::clock::Clock, turn: Color, frame: &mut Frame, area: Rect) {
    let [white_area, black_area] =
        Layout::horizontal(Constraint::from_percentages([50, 50])).areas(area);
    // let s = Style::new().black();
    let px = PixelSize::Quadrant;
    let (white_time, black_time) = clock.format();

    let white_text = BigText::builder()
        .centered()
        .pixel_size(px)
        .style(clock_style(Color::White, turn))
        .lines(vec![white_time.into()])
        .build();

    let black_text = BigText::builder()
        .centered()
        .pixel_size(px)
        .style(clock_style(Color::Black, turn))
        .lines(vec![black_time.into()])
        .build();

    let padding_top = (area.height - px_height(FONT_SIZE_CLOCK)) / 2;

    let white_block = Block::bordered()
        .style(clock_style(Color::White, turn))
        .padding(Padding::top(padding_top));

    let black_block = Block::bordered()
        .style(clock_style(Color::Black, turn))
        .padding(Padding::top(padding_top));

    frame.render_widget(&white_block, white_area);
    frame.render_widget(&black_block, black_area);
    frame.render_widget(white_text, white_block.inner(white_area));
    frame.render_widget(black_text, black_block.inner(black_area));
}

fn render_engine(
    game: &Chess,
    engine_move: &Option<Move>,
    waiting: bool,
    frame: &mut Frame,
    area: Rect,
) {
    let inner = if waiting {
        let n = (chrono::Utc::now().second() % 8) as usize;
        BigText::builder()
            .centered()
            .pixel_size(FONT_SIZE_ENGINE_MOVE)
            .style(
                Style::default()
                    .fg(UiColor::LightGreen)
                    .bg(UiColor::DarkGray),
            )
            .lines(vec![".".repeat(n).into()])
            .build()
    } else {
        match engine_move {
            None => BigText::builder()
                .centered()
                .pixel_size(PixelSize::Quadrant)
                .style(Style::default().fg(UiColor::White).bg(UiColor::DarkGray))
                .lines(vec![".".into()])
                .build(),
            Some(m) => BigText::builder()
                .centered()
                .pixel_size(FONT_SIZE_ENGINE_MOVE)
                .style(Style::default().fg(UiColor::Black).bg(UiColor::Gray))
                .lines(vec![san_format_move(game, m, true).into()])
                .build(),
        }
    };

    let block = match engine_move {
        None => Block::bordered()
            .padding(Padding::uniform(1))
            .bg(UiColor::DarkGray),
        Some(_) => Block::bordered()
            .padding(Padding::uniform(1))
            .bg(UiColor::Gray),
    };

    let inner_area = {
        let bi = block.inner(area);
        let top = bi.y + (bi.height / 2 - px_height(FONT_SIZE_ENGINE_MOVE) / 2);
        Rect::new(bi.x, top, bi.width, bi.height)
    };

    frame.render_widget(&block, area);
    frame.render_widget(inner, inner_area);
}

pub fn render(
    AppState {
        game,
        engine_move,
        engine_waiting,
        clock,
        avail_input,
        ..
    }: &AppState,
    frame: &mut Frame,
    area: Rect,
) {
    let [area_left, area_rigth] =
        Layout::horizontal([Constraint::Percentage(60), Constraint::Fill(1)]).areas(area);

    let [area_engine, area_clock] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(px_height(FONT_SIZE_CLOCK) * 3),
    ])
    .areas(area_rigth);

    render_engine(game, engine_move, *engine_waiting, frame, area_engine);
    render_clock(clock, game.turn(), frame, area_clock);

    if game.turn() == get_engine_color() {
        render_empty_input(frame, area_left);
    } else {
        crate::ui::input::render(game, *avail_input, frame, area_left);
    }
}

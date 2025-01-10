use chrono::Timelike;
use ratatui::style::{Color as UiColor, Style};
use ratatui::widgets::Padding;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::Stylize,
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

use shakmaty::{Chess, Color, Move, Position};
use tui_big_text::{BigText, PixelSize};

use crate::util::{i_to_alpha, san_format_move};

use super::AppState;
use super::{WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK};

static FONT_SIZE_CLOCK: PixelSize = PixelSize::Quadrant;
static FONT_SIZE_ENGINE_MOVE: PixelSize = PixelSize::Full;

fn px_height(px: PixelSize) -> u16 {
    // why its not public is beyond me...
    // pub(crate) fn pixels_per_cell(self) -> (u16, u16) {
    //     match self {
    //         PixelSize::Full => (1, 1),
    //         PixelSize::HalfHeight => (1, 2),
    //         PixelSize::HalfWidth => (2, 1),
    //         PixelSize::Quadrant => (2, 2),
    //         PixelSize::ThirdHeight => (1, 3),
    //         PixelSize::Sextant => (2, 3),
    //     }
    // }

    match px {
        PixelSize::Full => 8,
        PixelSize::HalfHeight => 8 / 2,
        PixelSize::HalfWidth => 8,
        PixelSize::Quadrant => 8 / 2,
        PixelSize::ThirdHeight => 8 / 3,
        PixelSize::Sextant => 8 / 3,
    }
}

#[derive(Debug, Clone)]
enum PossibleStart {
    Str(String),
    Piece(shakmaty::Role),
}

impl PossibleStart {
    fn span(self) -> Span<'static> {
        match self {
            PossibleStart::Str(s) => Span::raw(s),
            PossibleStart::Piece(role) => match role {
                shakmaty::Role::Pawn => Span::raw(format!("{WHITE_PAWN} ")),
                shakmaty::Role::Rook => Span::raw(format!("{WHITE_ROOK} ")),
                shakmaty::Role::Knight => Span::raw(format!("{WHITE_KNIGHT} ")),
                shakmaty::Role::Bishop => Span::raw(format!("{WHITE_BISHOP} ")),
                shakmaty::Role::Queen => Span::raw(format!("{WHITE_QUEEN} ")),
                shakmaty::Role::King => Span::raw(format!("{WHITE_KING} ")),
            },
        }
    }
}

#[derive(Debug, Clone)]
struct PossibleMove {
    id: String,
    mov: String,
    start: Option<PossibleStart>,
    selected: bool,
}

impl PossibleMove {
    fn spans(self) -> Vec<Span<'static>> {
        let start = self.start.clone();
        match (start, self.selected) {
            (None, false) => vec![
                Span::raw(format!(", {} ", &self.id)).italic(),
                Span::raw((&self.mov).to_string()).bold(),
            ],
            (None, true) => vec![
                Span::raw(format!(", {} ", &self.id)).italic(),
                Span::raw((&self.mov).to_string())
                    .bold()
                    .fg(UiColor::LightBlue),
            ],
            (Some(s), true) => vec![
                s.span(),
                Span::raw(format!("{} ", &self.id)).italic(),
                Span::raw((&self.mov).to_string())
                    .bold()
                    .fg(UiColor::LightBlue),
            ],
            (Some(s), false) => vec![
                s.span(),
                Span::raw(format!("{} ", &self.id)).italic(),
                Span::raw((&self.mov).to_string()).bold(),
            ],
        }
    }

    fn width(&self) -> u16 {
        self.clone().spans().iter().map(|s| s.width() as u16).sum()
    }
}

pub fn render_possible_moves(
    game: &Chess,
    avail_input: Option<usize>,
    frame: &mut Frame,
    area: Rect,
) {
    let mut lines: Vec<Vec<PossibleMove>> = vec![vec![], vec![], vec![], vec![], vec![], vec![]];

    for (i, m) in game.legal_moves().iter().enumerate() {
        let line = match m.role() {
            shakmaty::Role::Pawn => &mut lines[0],
            shakmaty::Role::Rook => &mut lines[1],
            shakmaty::Role::Knight => &mut lines[2],
            shakmaty::Role::Bishop => &mut lines[3],
            shakmaty::Role::Queen => &mut lines[4],
            shakmaty::Role::King => &mut lines[5],
        };

        line.push(PossibleMove {
            id: i_to_alpha(i),
            mov: san_format_move(game, m, false),
            selected: avail_input.map(|input| input == i).unwrap_or(false),
            start: if line.is_empty() {
                Some(PossibleStart::Piece(m.role()))
            } else {
                None
            },
        });
    }
    let mut text_content: Vec<Line> = Vec::new();
    let avail_space = area.width - 3;
    for spans in lines {
        let mut current_line = Line::default();
        // let mut first_line = true;
        let mut len = 0u16;
        for mut possible_move in spans.iter().cloned().collect::<Vec<_>>() {
            let slen = possible_move.width();
            if slen + len > avail_space {
                text_content.push(current_line);
                current_line = Line::raw("");
                possible_move.start = Some(PossibleStart::Str(" ↪ ".to_string()));
            }
            len = slen + (current_line.width() as u16);

            for s in possible_move.spans() {
                current_line.push_span(s);
            }
        }
        if current_line.width() > 0 {
            text_content.push(current_line);
        }
    }

    frame.render_widget(Paragraph::new(text_content).block(Block::bordered()), area);
}

fn render_empty_input(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Paragraph::new("Black to move").block(Block::bordered().title("input")),
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
    let [area_w, area_b] = Layout::horizontal(Constraint::from_percentages([50, 50])).areas(area);
    // let s = Style::new().black();
    let px = PixelSize::Quadrant;
    let (white, black) = clock.format();
    let w = BigText::builder()
        .centered()
        .pixel_size(px)
        .style(clock_style(Color::White, turn))
        .lines(vec![white.into()])
        .build();
    let b = BigText::builder()
        .centered()
        .pixel_size(px)
        .style(clock_style(Color::Black, turn))
        .lines(vec![black.into()])
        .build();

    let padding_top = (area.height - px_height(FONT_SIZE_CLOCK)) / 2;
    let block_w = Block::bordered()
        .style(clock_style(Color::White, turn))
        .padding(Padding::top(padding_top));
    let block_b = Block::bordered()
        .style(clock_style(Color::Black, turn))
        .padding(Padding::top(padding_top));
    frame.render_widget(&block_w, area_w);
    frame.render_widget(&block_b, area_b);
    frame.render_widget(w, block_w.inner(area_w));
    frame.render_widget(b, block_b.inner(area_b));
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

    if game.turn() == Color::White {
        render_possible_moves(game, *avail_input, frame, area_left);
    } else {
        render_empty_input(frame, area_left);
    }
}

use ratatui::layout::{Constraint, Rect};
use ratatui::style::Color as UiColor;
use ratatui::text::{Line, Span, ToSpan};
use ratatui::widgets::Padding;
use ratatui::{
    layout::Layout,
    prelude::Stylize,
    style::Style,
    widgets::{Block, Paragraph},
    Frame,
};
use shakmaty::san::San;
use shakmaty::{Board, Chess, Color, File, Move, Piece, Position, Rank, Square};
use tui_big_text::{BigText, PixelSize};

use crate::util::i_to_alpha;

pub const WHITE_PAWN: &str = "♙";
pub const WHITE_ROOK: &str = "♖";
pub const WHITE_KNIGHT: &str = "♘";
pub const WHITE_BISHOP: &str = "♗";
pub const WHITE_QUEEN: &str = "♕";
pub const WHITE_KING: &str = "♔";

pub const BLACK_PAWN: &str = "♟";
pub const BLACK_ROOK: &str = "♜";
pub const BLACK_KNIGHT: &str = "♞";
pub const BLACK_BISHOP: &str = "♝";
pub const BLACK_QUEEN: &str = "♛";
pub const BLACK_KING: &str = "♚";

fn render_pawn(color: Color) -> &'static str {
    match color {
        Color::Black => BLACK_PAWN,
        Color::White => WHITE_PAWN,
    }
}

fn render_rook(color: Color) -> &'static str {
    match color {
        Color::Black => BLACK_ROOK,
        Color::White => WHITE_ROOK,
    }
}

fn render_knight(color: Color) -> &'static str {
    match color {
        Color::Black => BLACK_KNIGHT,
        Color::White => WHITE_KNIGHT,
    }
}

fn render_bishop(color: Color) -> &'static str {
    match color {
        Color::Black => BLACK_BISHOP,
        Color::White => WHITE_BISHOP,
    }
}

fn render_queen(color: Color) -> &'static str {
    match color {
        Color::Black => BLACK_QUEEN,
        Color::White => WHITE_QUEEN,
    }
}

fn render_king(color: Color) -> &'static str {
    match color {
        Color::Black => BLACK_KING,
        Color::White => WHITE_KING,
    }
}

pub fn render_piece(piece: &Piece) -> &'static str {
    match piece.role {
        shakmaty::Role::Pawn => render_pawn(piece.color),
        shakmaty::Role::Rook => render_rook(piece.color),
        shakmaty::Role::Knight => render_knight(piece.color),
        shakmaty::Role::Bishop => render_bishop(piece.color),
        shakmaty::Role::Queen => render_queen(piece.color),
        shakmaty::Role::King => render_king(piece.color),
    }
}

pub fn render_board(board: &Board, frame: &mut Frame, area: Rect) {
    // println!("render board");
    let vsize: u16 = 1;
    let hsize: u16 = 2;
    let row_areas: [Rect; 8] = Layout::vertical([Constraint::Length(vsize); 8]).areas(area);
    let start: usize = 0;
    let end: usize = 7;
    for rank in start..=end {
        // let rank = end - irank;
        // println!("rank {rank}");
        let row_area = row_areas[end - rank];
        let square_areas: [Rect; 8] =
            Layout::horizontal([Constraint::Length(hsize); 8]).areas(row_area);
        for file in start..=end {
            // println!("file {file}");
            let square = Square::from_coords(File::new(file as u32), Rank::new(rank as u32));
            let color = if square.is_dark() {
                UiColor::Gray
            } else {
                UiColor::White
            };
            if let Some(piece) = board.piece_at(square) {
                let inner = Paragraph::new(render_piece(&piece))
                    .style(Style::new().fg(UiColor::Black))
                    .block(Block::new().bg(color));
                frame.render_widget(inner, square_areas[file]);
            } else {
                frame.render_widget(Block::new().bg(color), square_areas[file]);
            }
        }
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
                Span::raw(format!("{}", &self.mov)).bold(),
            ],
            (None, true) => vec![
                Span::raw(format!(", {} ", &self.id)).italic(),
                Span::raw(format!("{}", &self.mov))
                    .bold()
                    .fg(UiColor::LightBlue),
            ],
            (Some(s), true) => vec![
                s.span(),
                Span::raw(format!("{} ", &self.id)).italic(),
                Span::raw(format!("{}", &self.mov))
                    .bold()
                    .fg(UiColor::LightBlue),
            ],
            (Some(s), false) => vec![
                s.span(),
                Span::raw(format!("{} ", &self.id)).italic(),
                Span::raw(format!("{}", &self.mov)).bold(),
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
    // let mut lines = vec![
    //     vec![Span::raw(format!("{WHITE_PAWN} "))],
    //     vec![Span::raw(format!("{WHITE_ROOK} "))],
    //     vec![Span::raw(format!("{WHITE_KNIGHT} "))],
    //     vec![Span::raw(format!("{WHITE_BISHOP} "))],
    //     vec![Span::raw(format!("{WHITE_QUEEN} "))],
    //     vec![Span::raw(format!("{WHITE_KING} "))],
    //     ];

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
            mov: San::from_move(game, m).to_string(),
            selected: avail_input.map(|input| input == i).unwrap_or(false),
            start: if line.len() == 0 {
                Some(PossibleStart::Piece(m.role()))
            } else {
                None
            },
        });

        // if line.len() > 1 {
        //     line.push(Span::raw(format!(", {} ", i_to_alpha(i))).italic());
        // } else {
        //     line.push(Span::raw(format!("{} ", i_to_alpha(i))).italic());
        // }

        // match avail_input {
        //     Some(input) if input == i => {
        //         line.push(
        //             Span::raw(format!("{}", San::from_move(game, m).to_string()))
        //                 .bold()
        //                 .fg(UiColor::LightBlue),
        //         );
        //     }
        //     _ => {
        //         line.push(Span::raw(format!("{}", San::from_move(game, m).to_string())).bold());
        //     }
        // }
    }

    // let lines = game
    //     .legal_moves()
    //     .iter()
    //     .enumerate()
    //     .map(|(i, m)| Line::raw(format!("{}: {}", i, San::from_move(game, m).to_string())))
    //     .collect::<Vec<_>>();
    let mut text_content: Vec<Line> = Vec::new();
    let avail_space = area.width - 3;
    for spans in lines {
        let mut current_line = Line::default();
        // let mut first_line = true;
        let mut len = 0u16;
        for mut possible_move in spans.iter().map(|p| p.clone()).collect::<Vec<_>>() {
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
    // let flatlines: Vec<Line> = lines
    //     .iter()
    //     .map(|spans| Line::default().spans(spans.iter().map(|span| span.clone())))
    //     .collect();
    frame.render_widget(Paragraph::new(text_content).block(Block::bordered()), area);
}

fn render_empty_input(frame: &mut Frame, area: Rect) {
    frame.render_widget(Block::bordered(), area);
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
    let w = BigText::builder()
        .centered()
        .pixel_size(px)
        .style(clock_style(Color::White, turn))
        .lines(vec![clock.format(Color::White, turn).into()])
        .build();
    let b = BigText::builder()
        .centered()
        .pixel_size(px)
        .style(clock_style(Color::Black, turn))
        .lines(vec![clock.format(Color::Black, turn).into()])
        .build();

    let padding_size = 4;
    let block_w = Block::bordered()
        .style(clock_style(Color::White, turn))
        .padding(Padding::top(padding_size));
    let block_b = Block::bordered()
        .style(clock_style(Color::Black, turn))
        .padding(Padding::top(padding_size));
    frame.render_widget(&block_w, area_w);
    frame.render_widget(&block_b, area_b);
    frame.render_widget(w, block_w.inner(area_w));
    frame.render_widget(b, block_b.inner(area_b));
}

struct Turn<'a> {
    c: Chess,
    ml: &'a Vec<Move>,
    i: usize,
}

impl<'a> Turn<'a> {
    fn new(hist: &'a Vec<Move>) -> Self {
        Turn {
            ml: hist,
            c: Chess::new(),
            i: 0,
        }
    }

    fn with_outcome(&self, line: &str) -> String {
        if let Some(outcome) = self.c.outcome() {
            format!("{}\n   {}", line, outcome)
        } else {
            String::from(line)
        }
    }
    fn format_move(&self) -> String {
        let wm = self.ml.get(self.i);
        let bm = self.ml.get(self.i + 1);
        let n = (self.i / 2) + 1;
        match (wm, bm) {
            (Some(w), Some(b)) => {
                let np = self.c.clone().play(w).expect("turn move should be OK");
                self.with_outcome(&format!(
                    "{:>3}. {}\t{}",
                    n,
                    San::from_move(&self.c, w),
                    San::from_move(&np, b)
                ))
            }
            (Some(w), None) => {
                self.with_outcome(&format!("{:>3}. {}", n, San::from_move(&self.c, w)))
            }
            _ => self.with_outcome(""),
        }
    }

    fn step(&mut self) -> bool {
        if self.c.outcome().is_some() {
            false
        } else {
            let wm = self.ml.get(self.i);
            let bm = self.ml.get(self.i + 1);
            match (wm, bm) {
                (_, None) => false,
                (Some(w), Some(b)) => {
                    self.c = self.c.clone().play(w).expect("white move should be ok");
                    self.c = self.c.clone().play(b).expect("black move should be ok");
                    self.i += 2;
                    true
                }
                _ => panic!("that cannot be"),
            }
        }
    }
}

fn render_hist(hist: &Vec<Move>, frame: &mut Frame, area: Rect) {
    let mut turn = Turn::new(hist);
    let mut lines = vec![Line::raw(turn.format_move())];
    while turn.step() {
        lines.push(Line::raw(turn.format_move()));
    }
    frame.render_widget(Paragraph::new(lines).block(Block::bordered()), area);
}

fn render_engine(game: &Chess, engine_move: &Option<Move>, frame: &mut Frame, area: Rect) {
    let inner = match engine_move {
        None => BigText::builder()
            .centered()
            .pixel_size(PixelSize::Quadrant)
            .style(
                Style::default()
                    .fg(UiColor::LightBlue)
                    .bg(UiColor::DarkGray),
            )
            .lines(vec![". . .".into()])
            .build(),
        Some(m) => BigText::builder()
            .centered()
            .pixel_size(PixelSize::Full)
            .style(Style::default().fg(UiColor::Black).bg(UiColor::Gray))
            .lines(vec![format!("{}", San::from_move(game, &m)).into()])
            .build(),
    };

    let block = match engine_move {
        None => Block::bordered()
            .padding(Padding::uniform(1))
            .bg(UiColor::DarkGray),
        Some(_) => Block::bordered()
            .padding(Padding::uniform(1))
            .bg(UiColor::Gray),
    };

    frame.render_widget(&block, area);
    frame.render_widget(inner, block.inner(area));
}

pub fn render_main(
    game: &Chess,
    hist: &Vec<Move>,
    clock: &crate::clock::Clock,
    engine_move: &Option<Move>,
    avail_input: Option<usize>,
    frame: &mut Frame,
) {
    let root_area = frame.area();
    let [display_area, input_area] =
        Layout::horizontal(Constraint::from_mins([0, 20])).areas(root_area);
    let [input_engine, display_clock, input_moves] =
        Layout::vertical(Constraint::from_mins([10, 10, 0])).areas(input_area);

    let [input_board, display_hist] =
        Layout::vertical(Constraint::from_fills([1, 3])).areas(display_area);

    render_board(&game.board(), frame, input_board);
    render_engine(game, engine_move, frame, input_engine);
    if game.turn() == Color::White {
        render_possible_moves(game, avail_input, frame, input_moves);
    } else {
        render_empty_input(frame, input_moves);
    }
    render_clock(clock, game.turn(), frame, display_clock);
    render_hist(hist, frame, display_hist);
}

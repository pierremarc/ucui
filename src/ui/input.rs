use crate::util::{self, i_to_alpha, san_format_move};
use ratatui::style::{Color as UiColor, Style, Stylize};
use ratatui::widgets::{Padding, Paragraph};
use ratatui::{layout::Rect, widgets::Block, Frame};
use shakmaty::{Chess, Move, Position, Role};
use tui_big_text::BigText;

// #[derive(Debug, Clone)]
// enum PossibleStart {
//     None,
//     Str(String),
//     Piece(shakmaty::Role),
// }

// impl PossibleStart {
//     fn span(self) -> Span<'static> {
//         match self {
//             PossibleStart::Str(s) => Span::raw(s),
//             PossibleStart::Piece(role) => match role {
//                 shakmaty::Role::Pawn => Span::raw(format!("{WHITE_PAWN} ")),
//                 shakmaty::Role::Rook => Span::raw(format!("{WHITE_ROOK} ")),
//                 shakmaty::Role::Knight => Span::raw(format!("{WHITE_KNIGHT} ")),
//                 shakmaty::Role::Bishop => Span::raw(format!("{WHITE_BISHOP} ")),
//                 shakmaty::Role::Queen => Span::raw(format!("{WHITE_QUEEN} ")),
//                 shakmaty::Role::King => Span::raw(format!("{WHITE_KING} ")),
//             },
//         }
//     }
// }

#[derive(Debug, Clone)]
struct PossibleMove {
    id: String,
    mov: Move,
    selected: bool,
}

// impl PossibleMove {
//     fn spans(self) -> Vec<Span<'static>> {
//         let start = self.start.clone();
//         match (start, self.selected) {
//             (None, false) => vec![
//                 Span::raw(format!(", {} ", &self.id)).italic(),
//                 Span::raw(self.mov.to_string()).bold(),
//             ],
//             (None, true) => vec![
//                 Span::raw(format!(", {} ", &self.id)).italic(),
//                 Span::raw(self.mov.to_string())
//                     .bold()
//                     .fg(UiColor::LightBlue),
//             ],
//             (Some(s), true) => vec![
//                 s.span(),
//                 Span::raw(format!("{} ", &self.id)).italic(),
//                 Span::raw(self.mov.to_string())
//                     .bold()
//                     .fg(UiColor::LightBlue),
//             ],
//             (Some(s), false) => vec![
//                 s.span(),
//                 Span::raw(format!("{} ", &self.id)).italic(),
//                 Span::raw(self.mov.to_string()).bold(),
//             ],
//         }
//     }

//     fn width(&self) -> u16 {
//         self.clone().spans().iter().map(|s| s.width() as u16).sum()
//     }
// }

const MOVE_WIDTH: u16 = 16;
const MOVE_HEIGHT: u16 = 6;

fn render_move(game: &Chess, m: &PossibleMove, x: u16, y: u16, frame: &mut Frame) {
    let move_string = san_format_move(game, &m.mov, true);
    let color = if m.selected {
        UiColor::LightYellow
    } else {
        UiColor::White
    };
    // let area = Rect {
    //     x,
    //     y,
    //     width: MOVE_WIDTH,
    //     height: MOVE_HEIGHT,
    // };
    let top_area = Rect {
        x,
        y,
        width: MOVE_WIDTH,
        height: MOVE_HEIGHT / 2,
    };
    let bottom_area = Rect {
        x,
        y: y + MOVE_HEIGHT / 2,
        width: MOVE_WIDTH,
        height: 1,
    };
    let move_text = BigText::builder()
        .centered()
        .pixel_size(tui_big_text::PixelSize::Sextant)
        .style(
            Style::default()
                // .bg(bg)
                .fg(color),
        )
        .lines(vec![move_string.into()])
        .build();

    // frame.render_widget(
    //     Block::default(), // .bg(bg)
    //     area,
    // );
    frame.render_widget(move_text, top_area);
    frame.render_widget(
        Paragraph::new(vec![m.id.to_string().into()])
            .fg(color)
            .bold()
            .italic()
            .centered(),
        bottom_area,
    );
}

fn render_moves(
    game: &Chess,
    role: Role,
    moves: &[PossibleMove],
    frame: &mut Frame,
    area: Rect,
) -> u16 {
    use util::role as rl;
    let pad = 1u16;
    let border = Block::bordered()
        .title(rl::format(
            role,
            &[
                rl::space(),
                rl::name(),
                rl::string(" — "),
                rl::symbol(),
                rl::space(),
            ],
        ))
        // .bg(UiColor::Green)
        .padding(Padding::uniform(pad));

    let inner_area = border.inner(area);
    let len = moves.len() as u16;
    let row_count = inner_area.width / MOVE_WIDTH;
    let more = if len % row_count > 0 { 1 } else { 0 };
    let height = (((len / row_count) + more) * MOVE_HEIGHT) + 2 * pad;
    frame.render_widget(border, Rect { height, ..area });

    let mut x = 0;
    let mut y = 0;
    for m in moves {
        if x + MOVE_WIDTH > inner_area.width {
            x = 0;
            y += MOVE_HEIGHT;
        }
        render_move(game, m, x + inner_area.x, y + inner_area.y, frame);
        x += MOVE_WIDTH;
    }
    area.y + y + MOVE_HEIGHT
}

pub fn render(game: &Chess, avail_input: Option<usize>, frame: &mut Frame, area: Rect) {
    let mut lines: Vec<Vec<PossibleMove>> = vec![vec![], vec![], vec![], vec![], vec![], vec![]];

    for (i, m) in game.legal_moves().iter().enumerate() {
        let line = match m.role() {
            shakmaty::Role::Pawn => &mut lines[0],
            shakmaty::Role::Bishop => &mut lines[1],
            shakmaty::Role::Knight => &mut lines[2],
            shakmaty::Role::Rook => &mut lines[3],
            shakmaty::Role::Queen => &mut lines[4],
            shakmaty::Role::King => &mut lines[5],
        };

        line.push(PossibleMove {
            id: i_to_alpha(i),
            mov: m.clone(),
            selected: avail_input.map(|input| input == i).unwrap_or(false),
        });
    }
    let margin = 2;
    let mut window = area.clone();
    for i in 0..=5 {
        let moves = &lines[i];
        let role = match i {
            0 => shakmaty::Role::Pawn,
            1 => shakmaty::Role::Bishop,
            2 => shakmaty::Role::Knight,
            3 => shakmaty::Role::Rook,
            4 => shakmaty::Role::Queen,
            5 => shakmaty::Role::King,
            6_usize.. => unreachable!(),
        };
        if moves.len() > 0 {
            let new_y = render_moves(game, role, moves, frame, window);
            window = Rect {
                y: new_y + margin,
                // height: window.height - (new_y - window.y),
                ..window
            }
        }
    }
    // let mut text_content: Vec<Line> = Vec::new();
    // let avail_space = area.width - 3;
    // for spans in lines {
    //     let mut current_line = Line::default();
    //     // let mut first_line = true;
    //     let mut len = 0u16;
    //     for mut possible_move in spans.iter().cloned() {
    //         let slen = possible_move.width();
    //         if slen + len > avail_space {
    //             text_content.push(current_line);
    //             current_line = Line::raw("");
    //             possible_move.start = Some(PossibleStart::Str(" ↪ ".to_string()));
    //         }
    //         len = slen + (current_line.width() as u16);

    //         for s in possible_move.spans() {
    //             current_line.push_span(s);
    //         }
    //     }
    //     if current_line.width() > 0 {
    //         text_content.push(current_line);
    //     }
    // }

    // frame.render_widget(Paragraph::new(text_content).block(Block::bordered()), area);
}

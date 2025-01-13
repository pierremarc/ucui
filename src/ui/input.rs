use crate::util::{self, check_rect, i_to_alpha, san_format_move};
use ratatui::style::{Color as UiColor, Style, Stylize};
use ratatui::widgets::{Padding, Paragraph};
use ratatui::{layout::Rect, widgets::Block, Frame};
use shakmaty::{Chess, Move, Position, Role};
use tui_big_text::BigText;

#[derive(Debug, Clone)]
struct PossibleMove {
    id: String,
    mov: Move,
    selected: bool,
}

const MOVE_WIDTH: u16 = 18;
const MOVE_HEIGHT: u16 = 6;

fn render_move(game: &Chess, m: &PossibleMove, x: u16, y: u16, frame: &mut Frame, area: Rect) {
    let move_string = san_format_move(game, &m.mov, true);
    let color = if m.selected {
        UiColor::LightYellow
    } else {
        UiColor::White
    };

    let top_area = check_rect(
        area,
        Rect {
            x,
            y,
            width: MOVE_WIDTH,
            height: MOVE_HEIGHT / 2,
        },
    );
    let bottom_area = check_rect(
        area,
        Rect {
            x,
            y: y + MOVE_HEIGHT / 2,
            width: MOVE_WIDTH,
            height: 1,
        },
    );
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

    frame.render_widget(move_text, top_area);
    frame.render_widget(
        Paragraph::new(vec![m.id.to_string().into()])
            .fg(color)
            .bold()
            // .italic()
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
    frame.render_widget(border, check_rect(area, Rect { height, ..area }));

    let mut x = 0;
    let mut y = 0;
    for m in moves {
        if x + MOVE_WIDTH > inner_area.width {
            x = 0;
            y += MOVE_HEIGHT;
        }
        render_move(game, m, x + inner_area.x, y + inner_area.y, frame, area);
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
    let mut window = area;
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
        if !moves.is_empty() {
            let new_y = render_moves(game, role, moves, frame, window);
            window = check_rect(
                area,
                Rect {
                    y: new_y + margin,
                    ..window
                },
            )
        }
    }
}

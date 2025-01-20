use crate::state::State;
use crate::util::role::role_letter;
use crate::util::{self, check_rect, px_width, shrink_rect, MoveIndex, MoveMap, ROLE_LIST};
use ratatui::style::{Color as UiColor, Style, Stylize};
use ratatui::widgets::Padding;
use ratatui::{layout::Rect, widgets::Block, Frame};
use shakmaty::san::San;
use shakmaty::{Chess, Move, Position, Role};
use tui_big_text::BigText;

// #[derive(Debug, Clone)]
// struct PossibleMove {
//     id: String,
//     mov: Move,
//     selected: bool,
// }

// const MOVE_WIDTH: u16 = 18;
const MOVE_HEIGHT: u16 = 4;
const MOVE_PX_SIZE: tui_big_text::PixelSize = tui_big_text::PixelSize::Sextant;
const SELECTED_COLOR: UiColor = UiColor::LightYellow;
const NOT_SELECTED_COLOR: UiColor = UiColor::Reset;
const NO_MOVES_COLOR: UiColor = UiColor::DarkGray;

// fn render_move(game: &Chess, m: &PossibleMove, x: u16, y: u16, frame: &mut Frame, area: Rect) {
//     let move_string = san_format_move(game, &m.mov, true);
//     let color = if m.selected {
//         UiColor::LightYellow
//     } else {
//         UiColor::White
//     };

//     let top_area = check_rect(
//         area,
//         Rect {
//             x,
//             y,
//             width: MOVE_WIDTH,
//             height: MOVE_HEIGHT / 2,
//         },
//     );
//     let bottom_area = check_rect(
//         area,
//         Rect {
//             x,
//             y: y + MOVE_HEIGHT / 2,
//             width: MOVE_WIDTH,
//             height: 1,
//         },
//     );
//     let move_text = BigText::builder()
//         .centered()
//         .pixel_size(tui_big_text::PixelSize::Sextant)
//         .style(
//             Style::default()
//                 // .bg(bg)
//                 .fg(color),
//         )
//         .lines(vec![move_string.into()])
//         .build();

//     frame.render_widget(move_text, top_area);
//     frame.render_widget(
//         Paragraph::new(vec![m.id.to_string().into()])
//             .fg(color)
//             .bold()
//             // .italic()
//             .centered(),
//         bottom_area,
//     );
// }

// fn render_moves(
//     game: &Chess,
//     role: Role,
//     moves: &[PossibleMove],
//     frame: &mut Frame,
//     area: Rect,
// ) -> u16 {
//     use util::role as rl;
//     let pad = 1u16;
//     let border = Block::bordered()
//         .title(rl::format(
//             role,
//             &[
//                 rl::space(),
//                 rl::name(),
//                 rl::string(" — "),
//                 rl::symbol(),
//                 rl::space(),
//             ],
//         ))
//         // .bg(UiColor::Green)
//         .padding(Padding::uniform(pad));

//     let inner_area = border.inner(area);
//     let len = moves.len() as u16;
//     let row_count = inner_area.width / MOVE_WIDTH;
//     let more = if len % row_count > 0 { 1 } else { 0 };
//     let height = (((len / row_count) + more) * MOVE_HEIGHT) + 2 * pad;
//     frame.render_widget(border, check_rect(area, Rect { height, ..area }));

//     let mut x = 0;
//     let mut y = 0;
//     for m in moves {
//         if x + MOVE_WIDTH > inner_area.width {
//             x = 0;
//             y += MOVE_HEIGHT;
//         }
//         render_move(game, m, x + inner_area.x, y + inner_area.y, frame, area);
//         x += MOVE_WIDTH;
//     }
//     area.y + y + MOVE_HEIGHT
// }

fn render_input_move(
    move_string: &str,
    move_width: u16,
    color: UiColor,
    x: u16,
    y: u16,
    frame: &mut Frame,
    area: Rect,
) {
    let top_area = check_rect(
        area,
        Rect {
            x,
            y,
            width: move_width,
            height: MOVE_HEIGHT,
        },
    );

    let move_text = BigText::builder()
        .centered()
        .pixel_size(MOVE_PX_SIZE)
        .style(Style::default().fg(color))
        .lines(vec![move_string.into()])
        .build();

    frame.render_widget(move_text, top_area);
}

fn render_input_border(
    role: &Role,
    move_width: u16,
    len: u16,
    fg: UiColor,
    frame: &mut Frame,
    area: Rect,
) -> Rect {
    use util::role as rl;
    let pad = 1u16;
    let border = Block::bordered()
        .title(rl::format(
            *role,
            &[
                rl::space(),
                rl::name(),
                rl::string(" — "),
                rl::symbol(),
                rl::space(),
            ],
        ))
        .fg(fg)
        .padding(Padding::uniform(pad));

    let inner_area = border.inner(area);
    let row_count = inner_area.width / move_width;
    let more = if len % row_count > 0 { 1 } else { 0 };
    let height = (((len / row_count) + more) * MOVE_HEIGHT) + 2 * pad;
    frame.render_widget(border, check_rect(area, Rect { height, ..area }));

    inner_area
}

fn render_input_row(
    game: &Chess,
    role: &Role,
    moves: &Vec<(MoveIndex, Move)>,
    state: &State,
    frame: &mut Frame,
    area: Rect,
) -> u16 {
    let border_fg = match state.input {
        MoveIndex::Full(r, _) | MoveIndex::Role(r) if r == *role => SELECTED_COLOR,
        _ => UiColor::White,
    };

    let move_strings: Vec<String> = moves
        .iter()
        .map(|(_, m)| San::from_move(game, m).to_string())
        .collect();

    let max_move_width = move_strings
        .iter()
        .map(|s| (s.len() as u16) * px_width(MOVE_PX_SIZE))
        .max()
        .unwrap_or(1) as u16;
    let move_width = max_move_width + 4;
    let block_area =
        render_input_border(role, move_width, moves.len() as u16, border_fg, frame, area);

    let mut x = 0;
    let mut y = 0;
    for (i, (move_index, _)) in moves.into_iter().enumerate() {
        let color = if *move_index == state.input {
            SELECTED_COLOR
        } else {
            UiColor::White
        };
        if x + move_width > block_area.width {
            x = 0;
            y += MOVE_HEIGHT;
        }
        render_input_move(
            &move_strings[i],
            move_width,
            color,
            x + block_area.x,
            y + block_area.y,
            frame,
            area,
        );
        x += move_width;
    }
    area.y + y + MOVE_HEIGHT
}

fn get_role_color(selected: Option<&Role>, has_moves: bool, role: &Role) -> UiColor {
    let is_selected = move |r: &Role| selected.map(|s| *s == *r).unwrap_or(false);
    if !has_moves {
        return NO_MOVES_COLOR;
    };
    if is_selected(role) {
        return SELECTED_COLOR;
    };
    NOT_SELECTED_COLOR
}

fn render_pieces(
    move_map: &MoveMap,
    input: &MoveIndex,
    frame: &mut Frame,
    area: Rect,
) -> (Option<Role>, u16, u16) {
    let width = area.width / (ROLE_LIST.len() as u16);
    let mut selected_has_moves: Option<Role> = None;
    for (i, role) in ROLE_LIST.iter().enumerate() {
        let has_moves = !move_map.get_line(role).is_empty();
        let selected = match input {
            MoveIndex::Full(r, _) | MoveIndex::Role(r) if *r == *role => Some(r),
            _ => None,
        };
        if selected_has_moves.is_none() {
            selected_has_moves = selected.and_then(|r| if has_moves { Some(*r) } else { None });
        }
        let color = get_role_color(selected, has_moves, role);
        let border = Block::bordered().fg(color);
        let rect = Rect {
            x: (i as u16) * width + area.x,
            y: area.y,
            width,
            height: width / 2,
        };
        // let text = Paragraph::new(util::role::role_name(role).fg(color));
        let text = BigText::builder()
            // .lines([role_name(role).into()])
            .lines([role_letter(role).into()])
            .centered()
            .pixel_size(tui_big_text::PixelSize::HalfHeight)
            .style(Style::default().fg(color))
            .build();
        let inner_rect = border.inner(rect);
        frame.render_widget(border, rect);
        frame.render_widget(
            text,
            shrink_rect(
                inner_rect,
                crate::util::PaddingMod::Top(inner_rect.height / 3),
            ),
        );
    }
    (
        selected_has_moves,
        area.y + width / 2,
        (ROLE_LIST.len() as u16) * width,
    )
}

pub fn render(game: &Chess, state: &State, frame: &mut Frame, area: Rect) {
    let move_map = MoveMap::new(game.legal_moves().iter().map(Move::clone).collect());
    let margin = 2;

    if !move_map.is_empty() {
        let (selected_has_moves, y, width) = render_pieces(&move_map, &state.input, frame, area);
        if let Some(role) = selected_has_moves {
            let rect = check_rect(
                area,
                Rect {
                    y: y + margin,
                    width,
                    ..area
                },
            );
            let _ = render_input_row(game, &role, &move_map.get_line(&role), state, frame, rect);
        }
    } else if let Some(outcome) = game.outcome() {
        frame.render_widget(
            BigText::builder()
                .centered()
                .pixel_size(tui_big_text::PixelSize::Sextant)
                .style(Style::default())
                .lines(vec![outcome.to_string().into()])
                .build(),
            area,
        );
    }
}

// pub fn render(game: &Chess, avail_input: Option<usize>, frame: &mut Frame, area: Rect) {
//     let mut lines: Vec<Vec<PossibleMove>> = vec![vec![], vec![], vec![], vec![], vec![], vec![]];

//     for (i, m) in game.legal_moves().iter().enumerate() {
//         let line = match m.role() {
//             shakmaty::Role::Pawn => &mut lines[0],
//             shakmaty::Role::Bishop => &mut lines[1],
//             shakmaty::Role::Knight => &mut lines[2],
//             shakmaty::Role::Rook => &mut lines[3],
//             shakmaty::Role::Queen => &mut lines[4],
//             shakmaty::Role::King => &mut lines[5],
//         };

//         line.push(PossibleMove {
//             id: i_to_alpha(i),
//             mov: m.clone(),
//             selected: avail_input.map(|input| input == i).unwrap_or(false),
//         });
//     }
//     let margin = 2;
//     let mut window = area;
//     for i in 0..=5 {
//         let moves = &lines[i];
//         let role = match i {
//             0 => shakmaty::Role::Pawn,
//             1 => shakmaty::Role::Bishop,
//             2 => shakmaty::Role::Knight,
//             3 => shakmaty::Role::Rook,
//             4 => shakmaty::Role::Queen,
//             5 => shakmaty::Role::King,
//             6_usize.. => unreachable!(),
//         };
//         if !moves.is_empty() {
//             let new_y = render_moves(game, role, moves, frame, window);
//             window = check_rect(
//                 area,
//                 Rect {
//                     y: new_y + margin,
//                     ..window
//                 },
//             )
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use shakmaty::Role;

    #[test]
    fn check_piece_color_selected() {
        assert_eq!(
            get_role_color(Some(&Role::Pawn), true, &Role::Pawn),
            SELECTED_COLOR
        );
    }
    #[test]
    fn check_piece_color_nomoves() {
        assert_eq!(
            get_role_color(Some(&Role::Pawn), false, &Role::Pawn),
            NO_MOVES_COLOR
        );
    }
    #[test]
    fn check_piece_color_notselected() {
        assert_eq!(
            get_role_color(Some(&Role::King), true, &Role::Pawn),
            NOT_SELECTED_COLOR
        );
    }
    #[test]
    fn check_initial_role_list_with_pawn_index() {
        use ratatui::style::Color as UiColor;
        let input = MoveIndex::Role(Role::Pawn);
        let map = MoveMap::from_game(&Chess::default());
        let colors: Vec<UiColor> = ROLE_LIST
            .into_iter()
            .map(|role| {
                let moves = map.get_line(&role);
                match input {
                    MoveIndex::Role(r) | MoveIndex::Full(r, _) if r == role => {
                        get_role_color(Some(&role), !moves.is_empty(), &role)
                    }
                    _ => get_role_color(None, !moves.is_empty(), &role),
                }
            })
            .collect();
        assert_eq!(
            vec![
                SELECTED_COLOR,
                NO_MOVES_COLOR,
                NOT_SELECTED_COLOR,
                NO_MOVES_COLOR,
                NO_MOVES_COLOR,
                NO_MOVES_COLOR
            ],
            colors
        );
    }
}

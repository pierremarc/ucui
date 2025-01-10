use ratatui::style::{Color as UiColor, Style};
use ratatui::widgets::{Block, Paragraph};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::Stylize,
    Frame,
};

use shakmaty::{Board, Color, File, Piece, Rank, Square};

use super::{
    BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK, WHITE_BISHOP,
    WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
};

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
    let (width, height) = (hsize * 8, vsize * 8);
    let (left, top) = (
        area.x + (area.width / 2 - width / 2),
        area.y + (area.height / 2 - height / 2),
    );
    let area = Rect::new(left, top, width, height);
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

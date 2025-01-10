use ratatui::layout::Rect;
use shakmaty::{Chess, Move, Position};

const ALPHA: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

const ALPHA_START: u32 = 97;
const ALPHA_END: u32 = 122;

pub fn i_to_alpha(i: usize) -> String {
    let repeat = (i / 26) + 1;
    let index = i % 26;
    let c = ALPHA[index];
    (0..repeat).map(|_| c).collect()
}

pub fn alpha_to_i(a: &str) -> Result<usize, &str> {
    if let Some(c) = a.chars().next() {
        let repeat = a.len() - 1;
        let u = u32::from(c);
        if u >= ALPHA_START && u <= ALPHA_END {
            let i = (u - ALPHA_START) as usize;
            let r = 26 * repeat + i;
            return Ok(r);
        }
    }
    Err("failed to parse alpha")
}

pub fn san_format_move(pos: &Chess, m: &Move, already_played: bool) -> String {
    use shakmaty::san::San;
    let san_string = San::from_move(pos, m).to_string();
    let played = if already_played {
        Ok(pos.clone())
    } else {
        pos.clone().play(m)
    };
    match played {
        Err(_) => san_string,
        Ok(pos) => {
            if pos.is_checkmate() {
                return format!("{}#", san_string);
            } else if pos.is_check() {
                return format!("{}+", san_string);
            }
            san_string
        }
    }
}

pub enum PaddingMod {
    Top(u16),
    Right(u16),
    Bottom(u16),
    Left(u16),
}

fn u16add(a: u16, b: u16) -> u16 {
    a.checked_add(b).unwrap_or(u16::MAX)
}

fn u16min(a: u16, b: u16) -> u16 {
    a.checked_sub(b).unwrap_or(0)
}

pub fn shrink_rect(rect: Rect, padding: PaddingMod) -> Rect {
    match padding {
        PaddingMod::Top(n) => Rect {
            y: u16add(rect.y, n),
            ..rect
        },
        PaddingMod::Right(n) => Rect {
            width: u16min(rect.width, n),
            ..rect
        },
        PaddingMod::Bottom(n) => Rect {
            height: u16min(rect.height, n),
            ..rect
        },
        PaddingMod::Left(n) => Rect {
            x: u16add(rect.x, n),
            ..rect
        },
    }
}

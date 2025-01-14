use crossterm::style::SetBackgroundColor;
use ratatui::layout::Rect;
use shakmaty::{Chess, Move, Position, Role};
use std::{
    cmp::Ordering,
    collections::{linked_list, HashMap, LinkedList},
};
use tui_big_text::PixelSize;

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
        if (ALPHA_START..=ALPHA_END).contains(&u) {
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

pub struct MoveMap {
    source: Vec<Move>,
    moves: Vec<(Role, usize)>,
}

pub const ROLE_LIST: [Role; 6] = [
    Role::Pawn,
    Role::Bishop,
    Role::Knight,
    Role::Rook,
    Role::Queen,
    Role::King,
];

fn next_role_raw(r: Role) -> Role {
    match r {
        Role::Pawn => Role::Bishop,
        Role::Bishop => Role::Knight,
        Role::Knight => Role::Rook,
        Role::Rook => Role::Queen,
        Role::Queen => Role::King,
        Role::King => Role::Pawn,
    }
}
pub fn next_role(r: Role, map: &MoveMap) -> Option<Role> {
    let mut candidate = r;
    for _i in 0..ROLE_LIST.len() {
        candidate = next_role_raw(candidate);
        if !map.get_line(&candidate).is_empty() {
            return Some(candidate);
        }
    }
    None
}
fn prev_role_raw(r: Role) -> Role {
    match r {
        Role::Pawn => Role::King,
        Role::Bishop => Role::Pawn,
        Role::Knight => Role::Bishop,
        Role::Rook => Role::Knight,
        Role::Queen => Role::Rook,
        Role::King => Role::Queen,
    }
}

pub fn prev_role(r: Role, map: &MoveMap) -> Option<Role> {
    let mut candidate = r;
    for _i in 0..ROLE_LIST.len() {
        candidate = prev_role_raw(candidate);
        if !map.get_line(&candidate).is_empty() {
            return Some(candidate);
        }
    }
    None
}

pub fn next_index(len: usize, i: usize) -> usize {
    if len == 0 {
        0
    } else if i + 1 >= len {
        0
    } else {
        i + 1
    }
}

pub fn prev_index(len: usize, i: usize) -> usize {
    if len == 0 {
        0
    } else if i == 0 {
        len - 1
    } else {
        i - 1
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub enum MoveIndex {
    #[default]
    None,
    Role(Role),
    Full(Role, usize),
}

impl MoveMap {
    pub fn new(mut source: Vec<Move>) -> Self {
        (&mut source).sort_by(|a, b| {
            if a == b {
                Ordering::Equal
            } else {
                match (a, b) {
                    (Move::Put { .. }, Move::Put { .. }) => Ordering::Equal,
                    (Move::Put { .. }, _) => Ordering::Less,
                    (_, Move::Put { .. }) => Ordering::Greater,
                    (a, b) => match a.from().unwrap().cmp(&b.from().unwrap()) {
                        Ordering::Equal => a.to().cmp(&b.to()),
                        ord => ord,
                    },
                }
            }
        });

        let moves: Vec<(Role, usize)> = source
            .iter()
            .enumerate()
            .map(|(i, m)| (m.role(), i))
            .collect();

        Self { source, moves }
    }

    pub fn from_game(game: &Chess) -> Self {
        MoveMap::new(game.legal_moves().iter().map(Move::clone).collect())
    }

    pub fn get_line(&self, role: &Role) -> Vec<(MoveIndex, Move)> {
        let source = &self.source;
        self.moves
            .iter()
            .filter(|(r, _)| *r == *role)
            .enumerate()
            .map(|(line_index, (r, global_index))| {
                (
                    MoveIndex::Full(*r, line_index),
                    source[*global_index].clone(),
                )
            })
            .collect::<Vec<_>>()
    }

    pub fn get_move(&self, role: Role, index: usize) -> Option<Move> {
        let source = &self.source;
        self.moves
            .iter()
            .filter(|(r, _)| *r == role)
            .enumerate()
            .find(|(li, _)| *li == index)
            .map(|(_, (_, i))| source[*i].clone())
    }
}

pub enum PaddingMod {
    Top(u16),
    // Right(u16),
    // Bottom(u16),
    // Left(u16),
}

fn u16add(a: u16, b: u16) -> u16 {
    a.checked_add(b).unwrap_or(u16::MAX)
}

// fn u16min(a: u16, b: u16) -> u16 {
//     a.checked_sub(b).unwrap_or(0)
// }

pub fn shrink_rect(rect: Rect, padding: PaddingMod) -> Rect {
    match padding {
        PaddingMod::Top(n) => Rect {
            y: u16add(rect.y, n),
            ..rect
        },
        // PaddingMod::Right(n) => Rect {
        //     width: u16min(rect.width, n),
        //     ..rect
        // },
        // PaddingMod::Bottom(n) => Rect {
        //     height: u16min(rect.height, n),
        //     ..rect
        // },
        // PaddingMod::Left(n) => Rect {
        //     x: u16add(rect.x, n),
        //     ..rect
        // },
    }
}

pub fn px_height(px: PixelSize) -> u16 {
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

pub fn check_rect(base: Rect, candidate: Rect) -> Rect {
    let x = if candidate.x < base.x {
        base.x
    } else {
        candidate.x
    };
    let y = if candidate.y < base.y {
        base.y
    } else {
        candidate.y
    };
    let width = if x + candidate.width > base.x + base.width {
        base.width.saturating_sub(x)
    } else {
        candidate.width
    };
    let height = if y + candidate.height > base.y + base.height {
        base.height.saturating_sub(y)
    } else {
        candidate.height
    };
    Rect {
        x,
        y,
        width,
        height,
    }
}

pub mod role {
    use crate::ui::{WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK};
    use shakmaty::Role;

    fn role_symbol(role: &Role) -> &'static str {
        match role {
            shakmaty::Role::Pawn => WHITE_PAWN,
            shakmaty::Role::Rook => WHITE_ROOK,
            shakmaty::Role::Knight => WHITE_KNIGHT,
            shakmaty::Role::Bishop => WHITE_BISHOP,
            shakmaty::Role::Queen => WHITE_QUEEN,
            shakmaty::Role::King => WHITE_KING,
        }
    }

    fn role_name(role: &Role) -> &'static str {
        match role {
            shakmaty::Role::Pawn => "Pawn",
            shakmaty::Role::Rook => "Rook",
            shakmaty::Role::Knight => "Knight",
            shakmaty::Role::Bishop => "Bishop",
            shakmaty::Role::Queen => "Queen",
            shakmaty::Role::King => "King",
        }
    }

    pub enum RoleFormatItem {
        Space,
        Symbol,
        Name,
        String(String),
    }

    pub fn space() -> RoleFormatItem {
        RoleFormatItem::Space
    }
    pub fn name() -> RoleFormatItem {
        RoleFormatItem::Name
    }
    pub fn symbol() -> RoleFormatItem {
        RoleFormatItem::Symbol
    }
    pub fn string<S: Into<String>>(s: S) -> RoleFormatItem {
        RoleFormatItem::String(s.into())
    }

    pub fn format(role: Role, template: &[RoleFormatItem]) -> String {
        template
            .iter()
            .map(|i| match i {
                RoleFormatItem::Space => String::from(" "),
                RoleFormatItem::Name => role_name(&role).to_string(),
                RoleFormatItem::Symbol => role_symbol(&role).to_string(),
                RoleFormatItem::String(s) => s.clone(),
            })
            .collect()
    }
}

#[derive(Clone)]
pub struct RotatingList<T> {
    cap: usize,
    list: LinkedList<T>,
}

impl<T> RotatingList<T> {
    pub fn new(cap: usize) -> Self {
        RotatingList {
            cap,
            list: LinkedList::new(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let len = self.list.len();
        self.list.push_back(elem);
        if len >= self.cap {
            let _ = self.list.pop_front();
        }
    }

    pub fn iter(&self) -> linked_list::Iter<'_, T> {
        self.list.iter()
    }
}

// pub mod recv {
//     use std::{sync::mpsc::channel, thread::spawn};

//     use crossterm::event::Event;

//     use crate::state::StateValue;

//     enum MultiMessage {
//         State(StateValue),
//         Event(Event),
//     }
//     struct Multi {
//         rx: std::sync::mpsc::Receiver<MultiMessage>,
//     }

//     impl Multi {
//         fn new(
//             state: std::sync::mpsc::Receiver<StateValue>,
//             event: std::sync::mpsc::Receiver<Event>,
//         ) -> Self {
//             let (tx, rx) = channel::<MultiMessage>();
//             let tx1 = tx.clone();
//             spawn(move || loop {
//                 match state.recv() {
//                     Err(_) => break,
//                     Ok(m) => {
//                         let _ = tx1.send(MultiMessage::State(m));
//                     }
//                 }
//             });
//             let tx2 = tx.clone();
//             spawn(move || loop {
//                 match event.recv() {
//                     Err(_) => break,
//                     Ok(m) => {
//                         let _ = tx2.send(MultiMessage::Event(m));
//                     }
//                 }
//             });

//             Self { rx }
//         }

//         fn start(&self, tx: std::sync::mpsc::Sender<MultiMessage>) {
//             loop {
//                 match self.rx.recv() {
//                     Ok(m) => {
//                         let _ = tx.send(m);
//                     }
//                     Err(_) => break,
//                 }
//             }
//         }
//     }

//     pub fn multi(
//         state: std::sync::mpsc::Receiver<StateValue>,
//         event: std::sync::mpsc::Receiver<Event>,
//     ) -> std::sync::mpsc::Receiver<MultiMessage> {
//         let multi = Multi::new(state, event);
//         let (tx, rx) = channel::<MultiMessage>();

//         spawn(move || {
//             multi.start(tx);
//         });

//         rx
//     }
// }

use shakmaty::Move;
use std::cmp;
use std::collections::HashMap;
use std::sync::OnceLock;

pub struct Eco<'a> {
    pub code: &'a str,
    pub name: &'a str,
    // moves: &'a str,
}

impl<'a> Eco<'a> {
    fn new(code: &'a str, name: &'a str) -> Self {
        Eco { code, name }
    }
}

const MAX_MOVES: usize = 36;

static ECO_TABLE: OnceLock<HashMap<String, Eco>> = OnceLock::new();

fn init_table() -> HashMap<String, Eco<'static>> {
    let result: HashMap<String, Eco<'static>> = HashMap::from(include!("eco-table.in"));
    result
}

pub fn find_eco(mlist: &Vec<Move>) -> Option<&Eco<'_>> {
    let slen = cmp::min(MAX_MOVES, mlist.len());
    let range = 0..=slen;
    let ucis: Vec<String> = mlist
        .iter()
        .map(|m| format!("{}", m.to_uci(shakmaty::CastlingMode::Standard)))
        .collect();

    let table = ECO_TABLE.get_or_init(init_table);
    // make keys from longest to shortest
    let keys = range
        .rev()
        .map(|len| {
            (0..len)
                .map(|i| ucis.get(i).unwrap_or(&"----".to_string()).clone())
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>();

    for key in keys {
        if let Some(eco) = table.get(&key) {
            return Some(eco);
        }
    }
    None
}

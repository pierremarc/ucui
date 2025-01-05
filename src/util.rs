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

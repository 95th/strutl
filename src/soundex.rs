const SOUNDEX_MAP: &[u8] = b"01230120022455012623010202";

pub fn calc_soundex(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let s = clean(s);
    if s.is_empty() {
        return s;
    }

    let b = s.as_bytes();
    let first = b[0];
    let mut result = vec![first, b'0', b'0', b'0'];
    let mut last = map(first);
    let mut count = 1;

    for &c in &b[1..] {
        if count > 3 {
            break;
        }

        let c = match c {
            b'H' | b'W' => continue,
            c => map(c),
        };

        if c == b'-' {
            continue;
        } else if c != b'0' && c != last {
            result[count] = c;
            count += 1;
        }

        last = c;
    }

    String::from_utf8(result).unwrap()
}

#[inline]
fn map(c: u8) -> u8 {
    let i = c - b'A';
    SOUNDEX_MAP[i as usize]
}

#[inline]
fn clean(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if c.is_alphabetic() {
            out.push(c);
        }
    }
    out.to_uppercase()
}

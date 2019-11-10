use std::fmt;

const SOUNDEX_MAP: &[u8] = b"01230120022455012623010202";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Soundex {
    value: Option<[u8; 4]>,
}

impl AsRef<[u8]> for Soundex {
    fn as_ref(&self) -> &[u8] {
        if let Some(v) = &self.value {
            &v[..]
        } else {
            &[]
        }
    }
}

impl fmt::Display for Soundex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &c in self.as_ref() {
            write!(f, "{}", c as char)?;
        }
        Ok(())
    }
}

impl Soundex {
    pub fn new(s: &str) -> Soundex {
        Soundex {
            value: soundex(s.as_bytes()),
        }
    }
}

fn soundex(b: &[u8]) -> Option<[u8; 4]> {
    if b.is_empty() {
        return None;
    }

    let mut first = None;
    let mut last = 0;
    let mut result = [b'0', b'0', b'0', b'0'];
    let mut count = 1;
    for &c in b {
        if is_alphabetic(c) {
            let c = to_uppercase(c);
            if first.is_none() {
                first = Some(c);
                last = map(c);
                result[0] = c;
            } else if count > 3 {
                break;
            } else {
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
        }
    }

    if first.is_some() {
        Some(result)
    } else {
        None
    }
}

fn is_alphabetic(c: u8) -> bool {
    (b'a'..=b'z').contains(&c) || (b'A'..=b'Z').contains(&c)
}

fn to_uppercase(c: u8) -> u8 {
    match c {
        c @ b'a'..=b'z' => c - b'a' + b'A',
        _ => c,
    }
}

fn map(c: u8) -> u8 {
    let i = c - b'A';
    // Safety: since c is upper case alphabet, so 0 <= i < 26
    unsafe { *SOUNDEX_MAP.get_unchecked(i as usize) }
}

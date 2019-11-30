//! Conversion between integers and roman numerals.

#![no_std]

#[cfg(test)]
extern crate std;

extern crate alloc;
use alloc::string::String;

static ROMAN: &'static [(char, u16)] = &[
    ('I', 1),
    ('V', 5),
    ('X', 10),
    ('L', 50),
    ('C', 100),
    ('D', 500),
    ('M', 1000),
];
static ROMAN_PAIRS: &'static [(&'static str, u16)] = &[
    ("M", 1000),
    ("CM", 900),
    ("D", 500),
    ("CD", 400),
    ("C", 100),
    ("XC", 90),
    ("L", 50),
    ("XL", 40),
    ("X", 10),
    ("IX", 9),
    ("V", 5),
    ("IV", 4),
    ("I", 1),
];

/// The largest number representable as a roman numeral.
pub static MAX: u16 = 3999;

/// Converts an integer into a roman numeral.
///
/// Works for integer between 1 and 3999 inclusive, returns None otherwise.
///
/// # Example
///
/// ```
/// assert_eq!(roman::to(14), Some("XIV".to_string()));
/// assert_eq!(roman::to(0), None);
/// assert_eq!(roman::to(3999), Some("MMMCMXCIX".to_string()));
/// assert_eq!(roman::to(4000), None);
/// ```
pub fn to(n: u16) -> Option<String> {
    if n == 0 || n > MAX {
        return None;
    }
    let mut out = String::new();
    let mut n = n;
    for &(name, value) in ROMAN_PAIRS.iter() {
        while n >= value {
            n -= value;
            out.push_str(name);
        }
    }
    assert_eq!(n, 0);
    Some(out)
}

#[test]
fn test_to_roman() {
    let roman = "I II III IV V VI VII VIII IX X XI XII XIII XIV XV XVI XVII XVIII XIX XX XXI XXII"
        .split(' ');
    for (i, x) in roman.enumerate() {
        let n = (i + 1) as u16;
        assert_eq!(to(n).unwrap(), x);
    }
    assert_eq!(to(1984).unwrap(), "MCMLXXXIV");
}

/// Converts a roman numeral to an integer.
///
/// Works for integer between 1 and 3999 inclusive, returns None otherwise.
///
/// # Example
///
/// ```
/// assert_eq!(roman::from("XIV"), Some(14));
/// assert_eq!(roman::from(""), None);
/// ```
///
pub fn from(txt: &str) -> Option<u16> {
    let n = match from_lax(txt) {
        Some(n) => n,
        None => return None,
    };
    match to(n) {
        Some(ref x) if *x == txt => Some(n),
        _ => None,
    }
}

fn from_lax(txt: &str) -> Option<u16> {
    let (mut n, mut max) = (0, 0);
    for c in txt.chars().rev() {
        let it = ROMAN.iter().find(|x| {
            let &(ch, _) = *x;
            ch == c
        });
        let val = match it {
            Some(&(_, val)) => val,
            None => return None,
        };
        if val < max {
            n -= val;
        } else {
            n += val;
            max = val;
        }
    }
    Some(n)
}

#[test]
fn test_from() {
    assert!(from("i").is_none());
}

#[test]
fn test_to_from() {
    for n in 1..MAX {
        assert_eq!(from(&to(n).unwrap()).unwrap(), n);
    }
}

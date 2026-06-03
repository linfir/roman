#![doc = include_str!("../README.md")]

static ROMAN: &[(char, i32)] = &[
    ('I', 1),
    ('V', 5),
    ('X', 10),
    ('L', 50),
    ('C', 100),
    ('D', 500),
    ('M', 1000),
];
static ROMAN_PAIRS: &[(&str, i32)] = &[
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
pub static MAX: i32 = 3999;

/// Converts an integer into a roman numeral.
///
/// Works for integer between 1 and 3999 inclusive, returns None otherwise.
///
/// # Example
///
/// ```
/// let x = roman::to(14);
/// assert_eq!(x.unwrap(), "XIV");
/// ```
///
pub fn to(n: i32) -> Option<String> {
    if n <= 0 || n > MAX {
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
    assert!(n == 0);
    Some(out)
}

#[test]
fn test_to_roman() {
    let roman = "I II III IV V VI VII VIII IX X XI XII XIII XIV XV XVI XVII XVIII XIX XX XXI XXII"
        .split(' ');
    for (i, x) in roman.enumerate() {
        let n = (i + 1) as i32;
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
/// let x = roman::from("XIV");
/// assert_eq!(x.unwrap(), 14);
/// ```
///
pub fn from(txt: &str) -> Option<i32> {
    let n = from_lax(txt)?;
    match to(n) {
        Some(ref x) if *x == txt => Some(n),
        _ => None,
    }
}

fn from_lax(txt: &str) -> Option<i32> {
    let (mut n, mut max) = (0, 0);
    for c in txt.chars().rev() {
        let it = ROMAN.iter().find(|x| {
            let &(ch, _) = *x;
            ch == c
        });
        let &(_, val) = it?;
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

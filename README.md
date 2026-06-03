# roman

[![CI](https://github.com/linfir/roman.rs/actions/workflows/ci.yml/badge.svg)](https://github.com/linfir/roman.rs/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/roman.svg)](https://crates.io/crates/roman)
[![docs.rs](https://docs.rs/roman/badge.svg)](https://docs.rs/roman)
[![license](https://img.shields.io/crates/l/roman.svg)](LICENSE)

A Rust library to convert between integers and roman numerals.

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
roman = "0.1"
```

```rust
assert_eq!(roman::to(14),        Some("XIV".to_string()));
assert_eq!(roman::from("XIV"),   Some(14));
assert_eq!(roman::to(0),         None);  // out of range
assert_eq!(roman::from("XIIII"), None);  // non-canonical
```

Valid range: 1 to `roman::MAX` (3999).
Both functions return `None` outside that range.
`from` rejects non-canonical forms (e.g. `IIII` instead of `IV`).

//! # is-rust
//!
//! Have you ever wanted to know if a value is rust, or at least rusty?
//!
//! Maybe you're doing form validation for a Rust position at your company, and
//! you want to make sure that someone only inputs Rust-acceptable values. Say
//! that your form looks like this:
//!
//! ```ignore
//! 1: "What is your favourite programming language?"
//!
//! <answer>
//!
//! 2: "What language runs blazingly fast, prevents segfaults, and guarentees
//! thread safety?"
//!
//! <answer>
//!
//! 3: "What is your favourite colour defined in HEX?"
//!
//! <answer>
//!
//! 4: "When someone hasn't practiced something for a while, what would you call
//! them?"
//!
//! <answer>
//! ```
//!
//! Using typical form validation, you would have to define acceptable answers
//! yourself. Using this library, you can simply use the function `is_rust`
//! or `is_at_least_rusty` for all inputs.
//!
//! ### Examples
//!
//! For example, rust is rust:
//!
//! ```rust
//! extern crate is_rust;
//!
//! assert!(is_rust::is_rust("rust")); // this is rust
//! ```
//!
//! Similarly, Rust is rust:
//!
//! ```rust
//! extern crate is_rust;
//!
//! assert!(is_rust::is_rust("Rust")); // this is rust
//! ```
//!
//! It's all rust, except for strings that are not "rust", "b7410e", or the RGB
//! value `173, 65, 14`. Other than that, it's Rust. You can confirm this due to
//! the fact that Python is not Rust:
//!
//! ```rust
//! extern crate is_rust;
//!
//! assert!(!is_rust::is_rust("python")); // python is obviously not rust
//! ```
//!
//! Additionally you can check that something is rusty:
//!
//! ```rust
//! extern crate is_rust;
//!
//! assert!(is_rust::is_rusty("rusty"));
//! ```
//!
//! Something can be at least rusty, or rust itself:
//!
//! ```rust
//! extern crate is_rust;
//!
//! assert!(is_rust::is_at_least_rusty("rusty"));
//! assert!(is_rust::is_at_least_rusty("rust"));
//! ```
//!
//! You can check that a set of input is very rusty, ensuring that all input values
//! are rust:
//!
//! ```rust
//! extern crate is_rust;
//!
//! assert!(is_rust::is_very_rusty(&["rust", "Rust", "RUST", "b7410e"]));
//! ```
//!
//! Lastly, you can check that something is not rust, because booleans are hard:
//!
//! ```
//! extern crate is_rust;
//!
//! assert!(is_rust::is_not_rust("Python"));
//! ```
//!
//! At this point, Rust should no longer sound like a word.
//!
//! ### Installation
//!
//! To install this library to check whether values are Rust, you can use Rust's
//! Cargo. Add the following to your Rust project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! is-rust = "https://github.com/zeyla/is-rust"
//! ```
//!
//! ### License
//!
//! ISC.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use std::fmt::Display;

const NAME: &str = "rust";
const HEX: &str = "b7410e";
const HEX2: &str = "#b7410e";
const RUST_UNICODE: [&str; 20] = [
    "Ｒ",
    "ᴿ",
    "𝓡",
    "𝖱",
    "𝗥",
    "𝕽",
    "𝙍",
    "𝐑",
    "𝑅",
    "𝑹",
    "𝘙",
    "𝚁",
    "🄬",
    "🅁",
    "Ⓡ",
    "ℛ",
    "ℜ",
    "ℝ",
    "fe2o3",
    "rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.",
];
#[cfg(feature = "std")]
const RGB: [&str; 3] = ["173", "65", "14"];

/// Returns whether a value is rust.
///
/// This returns true if the value is `"rust"`, the hex value `"b7410e"`, or the
/// RGB value `173, 65, 14`.
///
/// # Examples
///
/// Rust should be rust:
///
/// ```rust
/// assert!(is_rust::is_rust("rust")); // is rust even a word anymore?
/// assert!(is_rust::is_rust("#b7410e"));
/// assert!(is_rust::is_rust("rgb(173, 65, 14)"));
/// ```
///
/// Python and Ruby are not rust:
///
/// ```rust
/// assert!(!is_rust::is_rust("python"));
/// assert!(!is_rust::is_rust("ruby"));
/// ```
#[cfg(feature = "std")]
pub fn is_rust<D: Display>(string: D) -> bool {
    _is_rust(&string.to_string())
}

#[cfg(feature = "std")]
fn _is_rust(string: &str) -> bool {
    str_is_rust(&string.to_lowercase())
}

/// Returns whether a value is rust.
///
/// Refer to the std-enabled version of this function.
///
/// This function does _not_ perform lowercasing.
#[cfg(not(feature = "std"))]
#[inline]
pub fn is_rust(string: &str) -> bool {
    str_is_rust(&string)
}

/// Returns whether the given value is equal to `"rusty"`.
///
/// This returns false if the value _is_ rust, because it's not "rust-y", it is
/// rust itself.
///
/// This does _not_ accept hex or RGB values _similar_ to rust's colour, because
/// then we might be getting too close to the colours of other languages, and
/// that's just not rusty.
///
/// # Examples
///
/// "rusty" is rusty:
///
/// ```rust
/// assert!(is_rust::is_rusty("rusty"));
/// ```
///
/// "rust" is not rusty:
///
/// ```rust
/// assert!(!is_rust::is_rusty("rust"));
/// ```
#[cfg(feature = "std")]
pub fn is_rusty<D: Display>(string: D) -> bool {
    _is_rusty(&string.to_string())
}

#[cfg(feature = "std")]
fn _is_rusty(string: &str) -> bool {
    ["rusty", "ferris"].contains(&&*string.to_lowercase())
}

/// Returns whether the given value is equal to `"rusty"`.
///
/// Refer to the std-enabled version of this function.
///
/// This function does _not_ perform lowercasing.
#[cfg(not(feature = "std"))]
pub fn is_rusty(string: &str) -> bool {
    ["rusty", "ferris"].contains(&string)
}

/// Returns whether a value is `"rusty"` or rust itself, meaning it is at least
/// rusty.
///
/// Refer to [`is_rust`] and [`is_rusty`].
///
/// # Examples
///
/// Rust and rusty values are both accepted:
///
/// ```rust
/// assert!(is_rust::is_at_least_rusty("rust"));
/// assert!(is_rust::is_at_least_rusty("rusty"));
/// ```
#[cfg(feature = "std")]
pub fn is_at_least_rusty<D: Display>(string: D) -> bool {
    _is_at_least_rusty(&string.to_string())
}

#[cfg(feature = "std")]
fn _is_at_least_rusty(string: &str) -> bool {
    let string = string.to_lowercase();

    is_rust(&string) || is_rusty(&string)
}

/// Returns whether a value is `"rusty"` or rust itself, meaning it is at least
/// rusty.
///
/// Refer to the std-enabled version of this function.
///
/// This function does _not_ perform lowercasing.
#[cfg(not(feature = "std"))]
pub fn is_at_least_rusty(string: &str) -> bool {
    is_rust(string) || is_rusty(string)
}

/// Returns whether all of the given values are rust.
///
/// Refer to [`is_rust`].
///
/// This function, despite the "rusty" suffix, is not related to [`is_rusty`].
///
/// # Examples
///
/// "rust", "Rust", "RUST", and "b7410e" are a very rusty combination:
///
/// ```rust
/// assert!(is_rust::is_very_rusty(&["rust", "Rust", "RUST", "b7410e"]));
/// ```
#[cfg(feature = "std")]
pub fn is_very_rusty<D: Display>(values: &[D]) -> bool {
    values.iter().all(|value| is_rust(value.to_string().to_lowercase()))
}

/// Returns whether all of the given values are rust.
///
/// Refer to the std-enabled version of this function.
///
/// This function does _not_ perform lowercasing.
#[cfg(not(feature = "std"))]
pub fn is_very_rusty(values: &[&str]) -> bool {
    values.iter().all(|value| is_rust(value))
}

/// Booleans are hard, so we provide a function to check that something is _not_
/// rust.
///
/// Refer to [`is_rust`] for how this function does _not_ work.
///
/// # Examples
///
/// C is not Rust:
///
/// ```rust
/// extern crate is_rust;
///
/// assert!(is_rust::is_not_rust("Python"));
/// ```
#[cfg(feature = "std")]
pub fn is_not_rust<D: Display>(string: D) -> bool {
    _is_not_rust(&string.to_string())
}

#[cfg(feature = "std")]
fn _is_not_rust(string: &str) -> bool {
    !is_rust(&string.to_lowercase())
}

/// Booleans are hard, so we provide a function to check that something is _not_
/// rust.
///
/// Refer to the std-enabled version of this function.
///
/// This function does _not_ perform lowercasing and does not check for RGB
/// values.
#[cfg(not(feature = "std"))]
#[inline]
pub fn is_not_rust(string: &str) -> bool {
    !is_rust(&string)
}

fn str_is_rust(mut s: &str) -> bool {
    if s == NAME || s == HEX || s == HEX2 {
        return true;
    }

    if let Some(pos) = s.find('(') {
        s = &s[pos + 1..];
    }

    if let Some(pos) = s.find(')') {
        s = &s[..pos];
    }

    if RUST_UNICODE.iter().any(|x| *x == s) {
        return true;
    }

    #[cfg(not(feature = "std"))]
    {
        return false;
    }

    #[cfg(feature = "std")]
    {
        let string = s.replace(' ', "");

        let parts = string.split(',').collect::<Vec<_>>();
        let part_count = parts.len();

        let mut cur = parts[0] == RGB[0] && parts[1] == RGB[1] && parts[2] == RGB[2];

        if part_count == 4 {
            cur = cur && parts[3].starts_with("1")
        }

        cur
    }
}

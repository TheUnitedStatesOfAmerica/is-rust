extern crate is_rust;

use is_rust::*;

#[test]
fn test_is_at_least_rusty() {
    assert!(is_at_least_rusty("rusty"));
    assert!(is_at_least_rusty("rust"));
    assert!(is_at_least_rusty("ruSt"));
}

#[test]
fn test_definitely_is_rust() {
    assert!(is_rust("rust"));
    assert!(is_rust("rust")); // check again just to be safe
    assert!(is_rust("rust")); // one more time
    assert!(is_rust("RUST"));
    assert!(is_rust("Rust"));
    assert!(is_rust("RusT"));
    assert!(is_rust("rusT"));
    assert!(is_rust("rUST"));
}

#[test]
fn test_definitely_is_rusty() {
    assert!(is_rusty("rusty"));
    assert!(is_rusty("RUSTY"));
    assert!(is_rusty("Rusty"));
    assert!(is_rusty("rustY"));
    assert!(is_rusty("rusty")); // check "rusty" again, you never know
    assert!(is_rusty("rusty")); // make sure a third time
}

#[test]
fn test_is_very_rusty() {
    assert!(is_very_rusty(&["rust", "rusT"]));
    assert!(!is_very_rusty(&["rust", "c"]));
}

#[test]
fn test_python_is_not_rust() {
    assert!(is_not_rust("python"));
}

#[test]
fn test_ruby_is_not_rust() {
    assert!(is_not_rust("ruby"));
}

#[test]
fn test_other_rusts() {
    assert!(is_rust("𝚁"));
    assert!(is_rust("Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety."));
    assert!(is_rust("rgb(173, 65, 14)"));
    assert!(!is_rust("rgb(173, 65, 11)"));
    assert!(is_rust("rgb(173, 65, 14, 1)"));
    assert!(!is_rust("rgb(173, 65, 14, 0)"));
}

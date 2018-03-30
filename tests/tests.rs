extern crate is_rust;

use is_rust::*;

#[test]
fn definitely_is_rust() {
    assert!(is_rust("rust"));
    assert!(is_rust("rust")); // check again just to be safe
    assert!(is_rust("RUST"));
    assert!(is_rust("Rust"));
    assert!(is_rust("RusT"));
    assert!(is_rust("rusT"));
    assert!(is_rust("rUST"));
}

#[test]
fn definitely_is_rusty() {
    assert!(is_rusty("rusty"));
    assert!(is_rusty("RUSTY"));
    assert!(is_rusty("Rusty"));
    assert!(is_rusty("rustY"));
    assert!(is_rusty("rusty")); // check "rusty" again, you never know
}

#[test]
fn python_is_not_rust() {
    assert!(is_not_rust("python"));
}

#[test]
fn ruby_is_not_rust() {
    assert!(is_not_rust("ruby"));
}

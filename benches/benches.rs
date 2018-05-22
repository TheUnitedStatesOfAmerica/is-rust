#![feature(test)]

extern crate is_rust;
extern crate test;

use self::test::Bencher;

#[bench]
fn is_rust(b: &mut Bencher) {
    b.iter(|| {
        is_rust::is_rust("rust");
        is_rust::is_rust("RusT");
        is_rust::is_rust("b7410e");
        is_rust::is_rust("#b7410e");
        is_rust::is_rust("fe2o3");
    });
}

#[bench]
fn is_rusty(b: &mut Bencher) {
    b.iter(|| {
        is_rust::is_rusty("rusty");
        is_rust::is_rusty("ferris");
    });
}

#[bench]
fn is_at_least_rusty(b: &mut Bencher) {
    b.iter(|| {
        is_rust::is_at_least_rusty("rust");
        is_rust::is_at_least_rusty("rusty");
    });
}

#[bench]
fn is_very_rusty(b: &mut Bencher) {
    b.iter(|| {
        is_rust::is_very_rusty(&["rust", "RUST"]);
    });
}

#[bench]
fn is_not_rust(b: &mut Bencher) {
    b.iter(|| {
        is_rust::is_not_rust("c++");
    });
}

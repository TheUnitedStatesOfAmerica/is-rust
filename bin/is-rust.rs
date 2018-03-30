extern crate is_rust;

use std::env::args;

fn main() {
    let mut correct = 0;
    let mut incorrect = 0;

    if args().len() == 1 {
        println!("Add the words you want to know the rustyness of as arguments.");

        return;
    }

    for arg in args().skip(1) {
        let symbol = if is_rust::is_at_least_rusty(&arg) {
            correct += 1;

            '✔'
        } else {
            incorrect += 1;

            'x'
        };

        println!("{} {}", symbol, arg);
    }

    println!("\n✔ {}; x {}", correct, incorrect);
}

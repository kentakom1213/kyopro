#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        L: usize,
        R: usize,
    }

    match (L == 1, R == 1) {
        (true, false) => println!("Yes"),
        (false, true) => println!("No"),
        _ => println!("Invalid"),
    }
}

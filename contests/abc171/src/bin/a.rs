#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        c: char
    }

    if c.is_uppercase() {
        println!("A");
    } else {
        println!("a");
    }
}

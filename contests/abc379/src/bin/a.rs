#![allow(non_snake_case)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: Chars
    }

    if let &[a, b, c, ..] = &N[..] {
        println!("{}{}{} {}{}{}", b, c, a, c, a, b);
    }
}

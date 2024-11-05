#![allow(non_snake_case)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut S: Chars
    }

    S.sort();

    if S == ['A', 'B', 'C'] {
        println!("Yes");
    } else {
        println!("No");
    }
}

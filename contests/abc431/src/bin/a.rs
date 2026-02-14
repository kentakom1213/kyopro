#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        H: usize,
        B: usize,
    }

    let ans = H.saturating_sub(B);

    println!("{ans}");
}


#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        R: usize,
        G: usize,
        B: usize,
        C: String
    }

    let ans = match &C[..] {
        "Red" => B.min(G),
        "Green" => R.min(B),
        _ => R.min(G),
    };

    println!("{ans}");
}

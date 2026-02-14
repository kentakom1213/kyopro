#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        S: [String; N],
    }

    let m = S.iter().map(|s| s.len()).max().unwrap();

    for s in S {
        let p = m - s.len();
        let pad = ".".repeat(p / 2);
        println!("{pad}{s}{pad}");
    }
}

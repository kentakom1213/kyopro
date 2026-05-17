#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        S: String,
        k: usize,
    }

    let N = S.len();

    println!("{}", &S[k..N - k]);
}

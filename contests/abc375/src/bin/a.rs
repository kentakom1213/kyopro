#![allow(non_snake_case)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
        S: Chars
    }

    if N <= 2 {
        println!("0");
        return;
    }

    let mut cnt = 0;

    for i in 0..N - 2 {
        if &S[i..i + 3] == &['#', '.', '#'] {
            cnt += 1;
        }
    }

    println!("{cnt}");
}

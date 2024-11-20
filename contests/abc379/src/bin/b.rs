#![allow(non_snake_case)]

use cp_library_rs::utils::run_length::RunLength;
use proconio::input;

fn main() {
    input! {
        N: usize,
        K: usize,
        S: String
    }

    let rle = S.chars().run_length_encode();

    let mut cnt = 0;

    for &(c, n) in &rle {
        if c == 'X' {
            continue;
        }
        cnt += n / K;
    }

    println!("{cnt}");
}

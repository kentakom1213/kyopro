#![allow(non_snake_case)]

use cp_library_rs::{debug, utils::run_length::run_length_encode_from};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        K: Usize1,
        S: String
    }

    let mut rle = run_length_encode_from(S.chars());

    debug!(rle);

    // K番目を入れ替える
    if rle[0].0 == '0' {
        rle.swap(2 * K, 2 * K + 1);
    } else {
        rle.swap(2 * K - 1, 2 * K);
    }

    debug!(rle);

    // joinする
    let ans = rle
        .iter()
        .map(|(c, n)| c.to_string().repeat(*n))
        .collect::<String>();

    println!("{}", ans);
}

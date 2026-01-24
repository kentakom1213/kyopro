#![allow(non_snake_case)]

use cp_library_rs::utils::run_length::RunLength;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        S: String
    }

    let rle = S.chars().map(|x| x as u8 - '0' as u8).run_length_encode();

    let mut ans = 0;

    for ((l, nl), (r, nr)) in rle.into_iter().tuple_windows() {
        if l + 1 == r {
            ans += nl.min(nr);
        }
    }

    println!("{ans}");
}

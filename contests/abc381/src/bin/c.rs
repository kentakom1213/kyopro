#![allow(non_snake_case)]

use cp_library_rs::{chmax, debug, utils::run_length::RunLength};
use proconio::input;

fn main() {
    input! {
        N: usize,
        S: String
    }

    // ランレングス圧縮
    let rle = S.chars().run_length_encode();

    debug!(rle);

    let mut ans = 0;

    if S.contains('/') {
        chmax!(ans, 1);
    }

    for win in rle.windows(3) {
        if let &[(a, x), (b, y), (c, z)] = win {
            if a == '1' && b == '/' && c == '2' && y == 1 {
                let len = 2 * x.min(z) + 1;
                chmax!(ans, len);
            }
        }
    }

    println!("{ans}");
}

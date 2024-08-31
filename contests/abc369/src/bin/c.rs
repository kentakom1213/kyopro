#![allow(non_snake_case)]

use cp_library_rs::{debug, utils::run_length::run_length_encode};
use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [isize; N]
    }

    if N == 1 {
        println!("1");
        return;
    }

    // 差分
    let mut D = vec![];
    for i in 1..N {
        D.push(A[i] - A[i - 1]);
    }
    debug!(D);

    // ランレングス圧縮
    let rle = run_length_encode(&D);

    debug!(rle);

    let mut ans = 0;

    for &(_, c) in &rle {
        let x = c + 1;
        ans += x * (x + 1) / 2;
    }

    ans -= rle.len() - 1;

    println!("{ans}");
}

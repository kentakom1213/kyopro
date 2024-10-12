#![allow(non_snake_case)]

use cp_library_rs::{chmin, debug, utils::consts::IINF};
use proconio::input;

fn main() {
    input! {
        N: usize,
        K: usize,
        mut A: [isize; N]
    }

    A.sort();

    let K = N - K;

    let mut res = IINF;

    for i in 0..N - K + 1 {
        let l = A[i];
        let r = A[i + K - 1];
        let tmp = r - l;
        debug!(l, r, &A[i..i + K]);
        chmin!(res, tmp);
    }

    println!("{res}");
}

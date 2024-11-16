#![allow(non_snake_case)]

use cp_library_rs::{chmin, utils::consts::INF};
use memoise::memoise;
use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [usize; N * 2]
    }

    let ans = dp(0, 2 * N, &A);

    println!("{ans}");
}

#[memoise(l <= 404, r <= 404)]
fn dp(l: usize, r: usize, A: &[usize]) -> usize {
    assert!(l < r, "{l} >= {r}");
    assert!((r - l) % 2 == 0, "{l}, {r}");

    if l + 2 == r {
        return A[l].abs_diff(A[l + 1]);
    }

    let mut ans = INF;

    // 外側から取る
    chmin!(ans, A[l].abs_diff(A[r - 1]) + dp(l + 1, r - 1, A));

    // ブロックに分割
    for i in (l + 2..r).step_by(2) {
        chmin!(ans, dp(l, i, A) + dp(i, r, A));
    }

    ans
}

#![allow(non_snake_case)]

use std::collections::BinaryHeap;

use cp_library_rs::{chmin, debug, utils::consts::INF};
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        T: usize,
    }

    for _ in 0..T {
        solve();
    }
}

fn solve() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N],
        B: [usize; N]
    }

    // Aの値でソート
    let (A, B): (Vec<_>, Vec<_>) = A.into_iter().zip(B.into_iter()).sorted().unzip();

    debug!(A, B);

    let mut Bk = B[..K].iter().cloned().collect::<BinaryHeap<_>>();
    let mut sum = B[..K].iter().sum::<usize>();

    let mut ans = sum * A[K - 1];

    for k in K..N {
        // Bの最大要素を取り出す
        let b_max = *Bk.peek().unwrap();

        let tmp = (sum + B[k] - b_max) * A[k];

        debug!(ans, tmp, k, A[k], B[k]);

        chmin!(ans, tmp);

        if b_max > B[k] {
            Bk.pop();
            Bk.push(B[k]);
            sum += B[k];
            sum -= b_max;
        }
    }

    println!("{ans}");
}

#![allow(non_snake_case)]

use cp_library_rs::{debug, utils::consts::INF};
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        T: usize
    }

    for _ in 0..T {
        solve();
    }
}

fn solve() {
    input! {
        N: usize,
        mut R: [usize; N]
    }

    for i in 0..N {
        R[i] = N - R[i];
    }

    debug!(R);

    let mut ans = 0;

    // 左から
    for i in 1..N {
        if R[i - 1] > R[i] {
            ans += R[i - 1] - R[i] - 1;
            R[i] = R[i - 1] - 1;
        }
    }

    debug!(ans);
    debug!(R);

    R.reverse();

    // 右から
    for i in 1..N {
        if R[i - 1] > R[i] {
            ans += R[i - 1] - R[i] - 1;
            R[i] = R[i - 1] - 1;
        }
    }

    debug!(R);

    println!("{ans}");
}

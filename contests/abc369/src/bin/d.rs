#![allow(non_snake_case)]

use cp_library_rs::{chmax, debug, utils::consts::IINF};
use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [isize; N]
    }

    let mut odd = vec![-IINF; N + 1];
    let mut even = vec![-IINF; N + 1];

    even[0] = 0;

    for i in 0..N {
        // 奇数回目
        chmax! {
            odd[i + 1],
            // 倒す
            even[i] + A[i],
            // 倒さない
            odd[i],
        };

        // 偶数回目
        chmax! {
            even[i + 1],
            // 倒す
            odd[i] + A[i] * 2,
            // 倒さない
            even[i],
        };
    }

    debug!(odd);
    debug!(even);

    let ans = odd[N].max(even[N]);

    println!("{ans}");
}

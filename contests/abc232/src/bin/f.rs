#![allow(non_snake_case)]

use cp_library_rs::{chmin, utils::consts::INF};
use proconio::input;

fn main() {
    input! {
        N: usize,
        X: usize,
        Y: usize,
        A: [usize; N],
        B: [usize; N]
    }

    // dp[S] := S(⊆A)をBの先頭|S|要素に対応させるときのコストの最小値
    let mut dp = vec![INF; 1 << N];
    dp[0] = 0;

    for S in 0_usize..1 << N {
        let j = S.count_ones() as usize;
        for i in 0..N {
            if S >> i & 1 == 1 {
                continue;
            }
            // 転倒数の計算
            // すでに追加された添字の中で，自分より大きいもの
            let cnt = (S & !((1 << i + 1) - 1)).count_ones() as usize;

            chmin! {
                dp[S | 1 << i],
                dp[S] + X * A[i].abs_diff(B[j]) + Y * cnt
            };
        }
    }

    println!("{}", dp.last().unwrap());
}

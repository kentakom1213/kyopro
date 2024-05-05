// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::BTreeMap;

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        Q: usize,
        TX: [(usize, usize); Q]
    }

    // 最終的な結果
    let mut ans = (N - 2) * (N - 2);

    // 更新操作を行う必要があるフレーム
    let mut H = N;
    let mut W = N;

    // A[r] := 行rのうち最も左にある白石の座標
    let mut A = vec![N; N + 1];
    // B[c] := 列cのうち最も上にある白石の座標
    let mut B = vec![N; N + 1];

    for &(t, x) in &TX {
        // 列xについて下方向
        if t == 1 {
            if x < W {
                // 枠の外に出る領域を更新
                for c in x..W {
                    B[c] = H;
                }
                W = x;
            }
            ans -= B[x] - 2;
        }
        // 行xについて右方向
        else {
            if x < H {
                // 枠の外に出る領域を更新
                for r in x..H {
                    A[r] = W;
                }
                H = x;
            }
            ans -= A[x] - 2;
        }
        // debug!(H, W, A, B);
    }

    println!("{ans}");
}

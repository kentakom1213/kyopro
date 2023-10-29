//               E - Gluttony
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc144/tasks/abc144_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
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
const INF: i64 = 1001001001001001001;

// main
fn main() {
    input! {
        N: usize,
        K: i64,
        mut A: [i64; N],
        mut F: [i64; N],
    }

    // Aは昇順
    A.sort();

    // Fは降順
    F.sort();
    F.reverse();

    // 判定関数
    // スコアxで目的を達成できるか
    let is_ok = |x: i64| -> bool {
        let mut cnt = 0;
        for i in 0..N {
            cnt += 0.max(A[i] - x / F[i]);
        }
        cnt <= K
    };

    // 二分探索
    let mut ng = -1;
    let mut ok = INF;

    while ok - ng > 1 {
        let mid = (ng + ok) / 2;
        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}

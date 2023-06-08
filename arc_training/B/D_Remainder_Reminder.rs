//          D - Remainder Reminder
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc090/tasks/arc091_b
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
const INF: usize = 1001001001001001001;

// main
fn main() {
    input! {
        N: usize,
        K: usize,
    }

    let mut ans = 0;

    // bを全探索
    for b in 1..=N {
        if b <= K {
            continue; // あまりがK以上になることはない
        }
        ans += (b - K) * (N / b);
        ans += (N % b + 1).saturating_sub(K);
        if K == 0 {
            ans -= 1;
        }
    }

    println!("{}", ans);
}

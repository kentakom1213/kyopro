//               EX23 - 3.03
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/apg4b/tasks/APG4b_bz
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
use superslice::Ext;

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
        mut A: [usize; N],
    }

    // 座標圧縮
    let comp = A.iter().sorted().dedup().cloned().collect_vec();

    // カウントする配列
    let mut cnt = vec![0; comp.len()];

    // カウント
    for a in &A {
        let idx = comp.lower_bound(a);
        cnt[idx] += 1;
    }

    // 答えを出力
    let ans = (0..comp.len()).max_by_key(|&i| cnt[i]).unwrap();
    println!("{} {}", comp[ans], cnt[ans]);
}

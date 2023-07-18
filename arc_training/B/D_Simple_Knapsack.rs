//           D - Simple Knapsack
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc060/tasks/arc073_b
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
        W: usize,
        wv: [(usize, usize); N],
    }

    // 重さの最小値
    let w1 = wv[0].0;

    // 荷物を分類
    let mut items = vec![vec![]; 4];
    let mut S = vec![vec![0]; 4];

    for &(w, v) in &wv {
        items[w - w1].push(v);
    }

    for i in 0..4 {
        // 荷物を価値の降順にソート
        items[i].sort();
        items[i].reverse();

        // 累積和をとる
        for j in 0..items[i].len() {
            let nxt = S[i].last().unwrap() + items[i][j];
            S[i].push(nxt);
        }
    }

    debug!(&items);
    debug!(&S);

    // 組合せを全探索
    let mut ans = 0;

    // w1, w1+1, w1+2, w1+3 をそれぞれ i,j,k,l 個選ぶ
    for i in 0..S[0].len() {
        for j in 0..S[1].len() {
            for k in 0..S[2].len() {
                for l in 0..S[3].len() {
                    if w1 * i + (w1 + 1) * j + (w1 + 2) * k + (w1 + 3) * l <= W {
                        ans = ans.max(S[0][i] + S[1][j] + S[2][k] + S[3][l]);
                    }
                }
            }
        }
    }

    println!("{}", ans);
}

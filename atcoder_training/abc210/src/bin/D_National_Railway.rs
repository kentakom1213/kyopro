//           D - National Railway
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc210/tasks/abc210_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1_000_000_000_000_000_000;

/// `a > b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmin {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

/// ## 方針
/// - DP
/// - 左上から右下に向かう
fn main() {
    input! {
        H: usize,
        W: usize,
        C: usize,
        mut A: [[usize; W]; H],
    }

    let mut ans = INF;

    for _ in 0..2 {
        // dp[i][j] := 片方の駅を設置し(i,j)にたどり着くまでにかかるコストの最小値
        let mut dp = vec![vec![INF; W]; H];
        for i in 0..H {
            for j in 0..W {
                // 駅を設置する
                chmin!(dp[i][j], A[i][j]);
                // 上から移動してくる
                if i > 0 {
                    chmin!(dp[i][j], dp[i - 1][j] + C);
                    chmin!(ans, dp[i - 1][j] + C + A[i][j]);
                }
                // 左から移動してくる
                if j > 0 {
                    chmin!(dp[i][j], dp[i][j - 1] + C);
                    chmin!(ans, dp[i][j - 1] + C + A[i][j]);
                }
            }
            // 左右を反転
            A[i].reverse();
        }
    }

    println!("{}", ans);
}

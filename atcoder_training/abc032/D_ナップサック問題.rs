//               D - ナップサック問題
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc032/tasks/abc032_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::BTreeSet;

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
const NEG1: usize = 1_usize.wrapping_neg();

/// `a < b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmax {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

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

// main
fn main() {
    input! {
        N: usize,
        W: usize,
        VW: [(usize, usize); N],
    }

    // 最大値で場合分け
    let vmax = VW.iter().map(|&(v, _)| v).max().unwrap();
    let wmax = VW.iter().map(|&(_, w)| w).max().unwrap();

    // 解を求める
    let ans = if vmax <= 1000 {
        solve_limited_value(N, W, &VW)
    } else if wmax <= 1000 {
        solve_limited_weight(N, W, &VW)
    } else {
        solve_limited_N(N, W, &VW)
    };

    println!("{}", ans);
}

/// N <= 30 の場合をとく
/// - 半分全列挙
fn solve_limited_N(N: usize, W: usize, VW: &[(usize, usize)]) -> usize {
    let left = &VW[..N / 2];
    let right = &VW[N / 2..];

    // 右半分を列挙
    let mut right_comb = BTreeSet::new();
    for i in 0..1 << right.len() {
        let (mut v, mut w) = (0, 0);
        for j in 0..right.len() {
            if i >> j & 1 == 1 {
                v += right[j].0;
                w += right[j].1;
            }
        }
        right_comb.insert((w, v));
    }

    // right_combを順に見ていき、vが単調増加になるように調整
    let mut vmax = 0;
    let right_comb: BTreeSet<_> = right_comb
        .iter()
        .map(|&(w, v)| {
            chmax!(vmax, v);
            (w, vmax)
        })
        .collect();

    // 左半分を列挙し、右半分との組合せを調べる
    let mut ans = 0;
    for i in 0..1 << left.len() {
        let (mut v, mut w) = (0, 0);
        for j in 0..left.len() {
            if i >> j & 1 == 1 {
                v += left[j].0;
                w += left[j].1;
            }
        }
        // 重さがWを超えない範囲で最大の価値を求める
        let tmp = right_comb.range(.. ((W + 1).saturating_sub(w), 0)).next_back();
        if let Some(&(ww, vv)) = tmp {
            chmax!(ans, v + vv);
        }
    }

    ans
}

/// w_i <= 1000 の場合をとく
fn solve_limited_weight(N: usize, W: usize, VW: &[(usize, usize)]) -> usize {
    // dp[i][j] := i番目までの品物から重さがj以下になるように選んだときの価値の最大値
    let mut dp = vec![vec![None; W + 1]; N + 1];
    dp[0][0] = Some(0);

    // 漸化式の計算
    for (i, &(v, w)) in VW.iter().enumerate() {
        for j in 0..=W {
            if dp[i][j].is_none() {
                continue;
            }

            // 選ばない場合
            chmax!(
                dp[i + 1][j],
                dp[i][j],
            );
            // 選ぶ場合
            if j + w <= W {
                chmax!(
                    dp[i + 1][j + w],
                    dp[i][j].map(|x| x + v),
                );
            }
        }
    }

    let ans = dp[N].iter().max().unwrap().unwrap();
    ans
}

/// v_i <= 1000 の場合をとく
fn solve_limited_value(N: usize, W: usize, VW: &[(usize, usize)]) -> usize {
    const V_MAX: usize = 200 * 1000 + 1;

    // dp[i][j] := 荷物iまでの中から価値がV以上になるように選んだときの重さの最小値
    let mut dp = vec![vec![INF; V_MAX]; N + 1];
    dp[0][0] = 0;

    // 漸化式
    for (i, &(v, w)) in VW.iter().enumerate() {
        for j in 0..V_MAX {
            // 選ばない場合
            chmin!(
                dp[i + 1][j],
                dp[i][j],
            );
            // 選ぶ場合
            if j + v < V_MAX {
                chmin!(
                    dp[i + 1][j + v],
                    dp[i][j] + w,
                );
            }
        }
    }

    let mut ans = 0;
    for i in 0..V_MAX {
        if dp[N][i] <= W {
            ans = i;
        }
    }
    ans
}

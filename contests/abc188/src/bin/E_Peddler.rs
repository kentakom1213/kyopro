//               E - Peddler               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc188/tasks/abc188_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::VecDeque;

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}};

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
const MINF: isize = -(INF as isize);

/// `chmax!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最大のものを、`x1`に代入する
/// - 代入があったとき、`true`を返す
#[macro_export]
macro_rules! chmax {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
    ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
        chmax! {
            $a,
            ($b).max($c)
            $(,$other)*
        }
    }}
}
/// `chmin!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最小のものを、`x1`に代入する
/// - 代入があったとき、`true`を返す
#[macro_export]
macro_rules! chmin {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
    ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
        chmin! {
            $a,
            ($b).min($c)
            $(,$other)*
        }
    }};
}

fn main() {
    input! {
        N: usize,
        M: usize,
        A: [isize; N],
        edges: [(Usize1, Usize1); M],
    }

    // 与えられるグラフはDAGである
    let mut G = vec![vec![]; N];
    for &(u, v) in &edges {
        G[u].push(v);
    }

    // 木DP
    // 逆向きに辿っていく
    let mut ans = MINF;

    // dp[i] := iの部分木の最大値
    let mut dp = vec![MINF; N];
    let mut visited = vec![false; N];

    for i in 0..N {
        dfs(i, &mut dp, &A, &mut visited, &G, &mut ans);
    }

    println!("{}", ans);
}

fn dfs(u: usize, dp: &mut Vec<isize>, A: &[isize], visited: &mut Vec<bool>, G: &Vec<Vec<usize>>, ans: &mut isize) {
    if visited[u] {
        return;
    }
    visited[u] = true;
    for &v in &G[u] {
        dfs(v, dp, A, visited, G, ans);
        chmax! {
            dp[u],
            dp[v]
        };
    }
    debug!(u, dp[u] - A[u], &dp);
    // 最小値を計算
    chmax! {
        *ans,
        dp[u] - A[u]
    };
    // 自分のノードを追加
    chmax! {
        dp[u],
        A[u],
    };
}

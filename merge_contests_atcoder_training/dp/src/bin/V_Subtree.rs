//               V - Subtree               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/dp/tasks/dp_v
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

const INF: usize = 1001001001001001001;

// main
fn main() {
    input! {
        N: usize,
        M: usize,
        edges: [(Usize1, Usize1); N - 1],
    }

    // グラフの構築
    let mut G = vec![vec![]; N];
    for &(u, v) in &edges {
        G[u].push(v);
        G[v].push(u);
    }

    // dp[i][j][k] := 頂点iを黒に塗るとき，頂点jを(k=0?白:黒)に塗るときの組合せの数
    let mut dp = vec![vec![vec![0; 2]; N]; N];

    for i in 0..N {
        rec(INF, i, &mut dp[i], &G, M);
        debug!(&dp[i]);
        println!("{}", dp[i][i][1]);
    }
}

fn rec(p: usize, u: usize, dp: &mut Vec<Vec<usize>>, G: &Vec<Vec<usize>>, M: usize) {
    dp[u][0] = 1;
    dp[u][1] = 1;

    // 葉である場合
    if G[u].len() == 1 && G[u][0] == p {
        return;
    }

    for &v in &G[u] {
        if v == p {
            continue;
        }
        rec(u, v, dp, G, M);

        // 白→白
        dp[u][0] *= dp[v][0];
        dp[u][0] %= M;

        // 白,黒→黒
        dp[u][1] *= dp[v][0] + dp[v][1];
        dp[u][1] %= M;
    }
}

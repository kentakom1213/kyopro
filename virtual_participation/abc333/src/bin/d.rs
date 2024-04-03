#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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

macro_rules! debug2D {
    ( $array:expr ) => {{
        #![cfg(debug_assertions)]
        eprintln!("{}: ", stringify!($array));
        for row in &$array {
            eprintln!("{:?}", row);
        }
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        UV: [(Usize1, Usize1); N - 1],
    }

    let G = UV.iter().fold(vec![vec![]; N], |mut g, &(u, v)| {
        g[u].push(v);
        g[v].push(u);
        g
    });

    // dp[i] := 頂点iを削除するのにかかる操作回数の合計の最小値
    let mut dp = vec![0; N];

    dfs(INF, 0, &G, &mut dp);
    debug!(dp);

    // 値が最大の頂点を除いて削除
    let mut max = 0;
    for &n in &G[0] {
        max = max.max(dp[n]);
    }

    let ans = dp[0] - max;

    println!("{ans}");
}

fn dfs(p: usize, u: usize, G: &Vec<Vec<usize>>, dp: &mut [usize]) {
    // 葉のとき
    if G[u].len() == 1 && G[u][0] == p {
        dp[u] = 1;
        return;
    }
    // 子を評価
    let mut sum = 0; // 子の和
    for &v in &G[u] {
        if v == p {
            continue;
        }
        dfs(u, v, G, dp);
        sum += dp[v];
    }
    // 自分
    dp[u] = sum + 1;
}

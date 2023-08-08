//                E - League               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc139/tasks/abc139_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::{collections::{BTreeSet, VecDeque}, process::exit};

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

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
    ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
        chmax! {
            $a,
            ($b).max($c)
            $(,$other)*
        }
    }}
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// main
fn main() {
    input! {
        N: usize,
        A: [[Usize1; N - 1]; N],
    }

    // 頂点の数
    let V = N * (N - 1) / 2 + 1;

    // 人のペアを整数になおす関数
    let flat = |mut i: usize, mut j: usize| -> usize {
        if i < j {
            std::mem::swap(&mut i, &mut j);
        }
        i * (i - 1) / 2 + j + 1
    };

    // グラフの構築
    let mut G = vec![vec![]; V];

    for i in 0..N {
        let mut cur = 0;
        for j in 0..N - 1 {
            let nxt = flat(i, A[i][j]);
            G[cur].push(nxt);
            cur = nxt;
        }
    }

    debug!(&G);

    // 閉路検出・最長路をDFSで行う
    let mut visited = vec![0; V];
    let mut dist = vec![0; V];
    let ans = dfs(0, &G, &mut visited, &mut dist);

    println!("{}", ans);
}

fn dfs(u: usize, G: &Vec<Vec<usize>>, visited: &mut Vec<usize>, dist: &mut Vec<usize>) -> usize {
    debug!(u, &visited, &dist);

    // 探索済みだった場合
    if visited[u] == 2 {
        return dist[u];
    }
    // ステータスを探索中に
    visited[u] = 1;
    // 次の頂点に進む
    for &v in &G[u] {
        // 閉路の検出
        if visited[v] == 1 {
            println!("-1");
            exit(0);
        }
        // 最長路の更新
        chmax!(
            dist[u],
            dfs(v, G, visited, dist) + 1
        );
    }
    // ステータスを探索済みに
    visited[u] = 2;
    // 現時点での最長路の長さを返す
    dist[u]
}

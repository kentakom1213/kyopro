//         F - Playing Tag on Tree
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc148/tasks/abc148_f
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
type Graph = Vec<Vec<usize>>;

// main
fn main() {
    input! {
        N: usize,
        t: Usize1, // 高橋くん
        a: Usize1, // 青木くん
        edges: [(Usize1, Usize1); N - 1],
    }

    // グラフの構築
    let mut G = vec![vec![]; N];
    for &(x, y) in &edges {
        G[x].push(y);
        G[y].push(x);
    }

    // 青木くんからの距離、高橋くんからの距離を計算
    let mut dist_t = bfs(t, &G);
    let mut dist_a = bfs(a, &G);

    debug!(&dist_t, &dist_a);

    let ans = dist_t
        .iter()
        .zip(dist_a.iter())
        .filter(|&(t, a)| t < a)
        .max_by_key(|&(t, a)| a)
        .unwrap();

    println!("{:?}", ans.1.saturating_sub(1));
}

fn bfs(start: usize, G: &Graph) -> Vec<usize> {
    let mut N = G.len();
    let mut q = VecDeque::new();
    q.push_back(start);
    let mut dist = vec![INF; N];
    dist[start] = 0;
    while let Some(u) = q.pop_front() {
        for &v in &G[u] {
            if dist[v] == INF {
                q.push_back(v);
                dist[v] = dist[u] + 1;
            }
        }
    }
    dist
}

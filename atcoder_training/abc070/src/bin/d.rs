// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::VecDeque;

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}, fastout};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

const INF: usize = 1001001001001001001;

#[fastout]
fn main() {
    input! {
        N: usize,
        edges: [(Usize1, Usize1, usize); N - 1],
        Q: usize,
        K: Usize1,
        queries: [(Usize1, Usize1); Q],
    }

    // グラフの構築
    let mut G = vec![vec![]; N];

    for &(u, v, w) in &edges {
        G[u].push((v, w));
        G[v].push((u, w));
    }

    // 頂点KからBFS
    let mut dist = vec![INF; N];
    dist[K] = 0;
    let mut q = VecDeque::new();
    q.push_back(K);

    while let Some(u) = q.pop_front() {
        for &(v, w) in &G[u] {
            if dist[v] == INF {
                dist[v] = dist[u] + w;
                q.push_back(v);
            }
        }
    }

    debug!(dist);

    // クエリ処理
    for &(u, v) in &queries {
        println!("{}", dist[u] + dist[v]);
    }
}

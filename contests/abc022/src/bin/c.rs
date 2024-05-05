// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1}, fastout,
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

use std::cmp::Reverse;
use std::collections::BinaryHeap;
/// Dijkstra法
/// - グラフ`graph`が与えられたとき、スタート地点`s`から各頂点への最短路を求める
pub fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, s: usize, omit: usize) -> Vec<usize> {
    const INF: usize = 1001001001001001001;
    let n: usize = graph.len();
    let mut dist: Vec<usize> = vec![INF; n];
    let mut pq: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();
    // 初期化
    dist[s] = 0;
    pq.push(Reverse((dist[s], s)));
    // 更新
    while let Some(Reverse((cost, u))) = pq.pop() {
        if dist[u] < cost {
            continue;
        }
        for &(v, weight) in &graph[u] {
            // 頂点0,omitは使わない
            if u == 0 && v == omit {
                continue;
            }
            if dist[v] > dist[u] + weight {
                dist[v] = dist[u] + weight;
                pq.push(Reverse((dist[v], v)));
            }
        }
    }
    dist
}

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        edges: [(Usize1, Usize1, usize); M],
    }

    let G = {
        let mut g = vec![vec![]; N];
        for &(u, v, w) in &edges {
            g[u].push((v, w));
            g[v].push((u, w));
        }
        g
    };

    // 各頂点からダイクストラ
    let mut ans = INF;

    for &(u, w) in &G[0] {
        let dist = dijkstra(&G, 0, u);
        debug!(u, dist);
        ans = ans.min(w + dist[u]);
    }

    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

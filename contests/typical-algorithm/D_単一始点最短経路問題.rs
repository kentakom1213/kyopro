//              D - 単一始点最短経路問題             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical-algorithm/tasks/typical_algorithm_d
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

use std::collections::BinaryHeap;
use std::cmp::Reverse;

// Dijkstra法
fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, s: usize) -> Vec<usize> {
    const INF: usize = 1001001001001001001;
    let n: usize = graph.len();
    let mut dist: Vec<usize> = vec![INF; n];
    let mut pq: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();
    // 初期化
    dist[s] = 0;
    pq.push(Reverse((dist[s], s)));
    // 更新
    while let Some(Reverse((cost, u))) = pq.pop() {
        // if dist[u] < cost { continue; }
        for &(v, weight) in &graph[u] {
            if dist[v] > dist[u] + weight {
                dist[v] = dist[u] + weight;
                pq.push(Reverse((dist[v], v)));
            }
        }
    }
    dist
}


// constant
const INF: usize = 1001001001001001001;

// main
fn main() {
    input! {
        N: usize,
        M: usize,
        edges: [(usize, usize, usize); M],
    }

    let mut G = vec![vec![]; N];
    for &(u, v, c) in &edges {
        G[u].push((v, c));
    }

    let dist = dijkstra(&G, 0);

    println!("{}", dist[N - 1]);
}

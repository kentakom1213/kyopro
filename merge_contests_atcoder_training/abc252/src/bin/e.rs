// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::HashMap;
// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

const INF: usize = 1001001001001001001;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
/// Dijkstra法
/// - グラフ`graph`が与えられたとき、スタート地点`s`から各頂点への最短路を求める
pub fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, s: usize) -> Vec<usize> {
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
            if dist[v] > dist[u] + weight {
                dist[v] = dist[u] + weight;
                pq.push(Reverse((dist[v], v)));
            }
        }
    }
    dist
}

fn main() {
    input! {
        N: usize,
        M: usize,
        edges: [(Usize1, Usize1, usize); M],
    }

    // グラフの構築
    let mut G = vec![vec![]; N];
    let mut edge_id = HashMap::new();

    for (i, &(u, v, w)) in edges.iter().enumerate() {
        G[u].push((v, w));
        G[v].push((u, w));
        edge_id.insert((u, v), i + 1);
        edge_id.insert((v, u), i + 1);
    }

    // ダイクストラ法
    let dist = dijkstra(&G, 0);
    debug!(&dist);

    // 逆向きに辿って最短路木を構築する
    let mut is_visited = vec![false; N];
    is_visited[0] = true;

    let mut ans = vec![];

    for i in 1..N {
        // 0~iの経路を復元
        let mut cur = i;
        while !is_visited[cur] {
            for &(prv, w) in &G[cur] {
                if dist[prv] + w == dist[cur] {
                    // 辺を採用
                    ans.push(edge_id[&(prv, cur)]);
                    is_visited[cur] = true;
                    cur = prv;
                    break;
                }
            }
        }
    }

    println!("{}", ans.iter().join(" "));
}

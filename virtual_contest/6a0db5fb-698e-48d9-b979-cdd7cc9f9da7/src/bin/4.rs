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
use rustc_hash::FxHashMap;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;


#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        edges: [(Usize1, Usize1, usize); M]
    }

    // 編の検索
    let edges = edges.iter().fold(FxHashMap::default(), |mut map, &(u, v, w)| {
        chmin! {
            *map.entry((u, v)).or_insert(INF),
            w
        };
        map
    });

    // グラフの構築
    let G = edges.iter().fold(vec![vec![]; N], |mut g, (&(u, v), &w)| {
        g[u].push((v, w));
        g
    });

    for s in 0..N {
        // 各都市への最短経路
        let dist = dijkstra(&G, s);
        // 最短経路
        let mut ans = INF;

        for g in 0..N {
            if let Some(&w) = edges.get(&(g, s)) {
                chmin! {
                    ans,
                    dist[g] + w
                };
            }
        }

        if ans == INF {
            println!("-1");
        } else {
            println!("{}", ans);
        }
    }
}

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

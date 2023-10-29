//                E - Skiing               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc237/tasks/abc237_e
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
use std::collections::BinaryHeap;
use std::cmp::Reverse;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

fn debug_2d<T: std::fmt::Debug>(array: &Vec<Vec<T>>) {
    #[cfg(debug_assertions)]
    for row in array {
        println!("{:?}", row);
    }
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// Dijkstra法
fn dijkstra(graph: &Vec<Vec<(usize, isize)>>, s: usize) -> Vec<isize> {
    const INF: isize = 1001001001001001001;
    let n: usize = graph.len();
    let mut dist: Vec<isize> = vec![INF; n];
    let mut pq: BinaryHeap<Reverse<(isize, usize)>> = BinaryHeap::new();
    // 初期化
    dist[s] = 0;
    pq.push(Reverse((dist[s], s)));
    // 更新
    while let Some(Reverse((cost, u))) = pq.pop() {
        if dist[u] < cost { continue; }
        for &(v, weight) in &graph[u] {
            if dist[v] > dist[u] + weight {
                dist[v] = dist[u] + weight;
                pq.push(Reverse((dist[v], v)));
            }
        }
    }
    dist
}


/// ## 考察
/// - 辺の重みを低い方から高い方に，その絶対値分だけ付与する
/// - H0からHiに到達するとき，えられる楽しさの最大値は，
///   Hi - H0 - (A0からAiへの最短経路)
fn main() {
    input! {
        N: usize,
        M: usize,
        H: [isize; N],
        edges: [(Usize1, Usize1); M],
    }

    // グラフの構築
    let mut G = vec![vec![]; N];
    for &(mut u, mut v) in &edges {
        if H[u] < H[v] {
            std::mem::swap(&mut u, &mut v)
        }
        // 上→下（コストなし）
        G[u].push((v, 0));
        // 下→上（コストあり）
        G[v].push((u, H[u] - H[v]));
    }

    debug_2d(&G);

    // 最短経路探索
    let cost = dijkstra(&G, 0);

    // 答えを求める
    let mut ans = 0;
    for i in 0..N {
        ans = ans.max(
            H[0] - H[i] - cost[i]
        );
    }

    println!("{}", ans);
}

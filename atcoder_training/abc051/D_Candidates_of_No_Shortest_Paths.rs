//   D - Candidates of No Shortest Paths   
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc051/tasks/abc051_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

// imports
use std::collections::{HashMap, BTreeMap, VecDeque, BinaryHeap};
use std::cmp::Reverse;

// input macro
// [Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
macro_rules! get {
    ($t:ty) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($($t:ty),*) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
    ($t:ty ; $n:expr) => {
        (0..$n).map(|_|
            get!($t)
        ).collect::<Vec<_>>()
    };
    ($($t:ty),* ; $n:expr) => {
        (0..$n).map(|_|
            get!($($t),*)
        ).collect::<Vec<_>>()
    };
    ($t:ty ;;) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    ($t:ty ;; $n:expr) => {
        (0..$n).map(|_|
            get!($t ;;)
        ).collect::<Vec<_>>()
    };
}


// solve
fn main() {
    let (N, M) = get!(usize, usize);
    const INF: usize = 1001001001001;
    let mut G = vec![vec![]; N];
    for i in 0..M {
        let (a, b, c) = get!(usize, usize, usize);
        G[a-1].push((b-1, c));
        G[b-1].push((a-1, c));
    }

    // 全頂点間でダイクストラ法
    for i in 0..N {
        let dist = dijkstra(&G, i);
        println!("{} : {:?}", i, dist);
    }
}

// Dijkstra法
fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, s: usize) -> Vec<usize> {
    const INF: usize = 1001001001001001001;
    let n: usize = graph.len();
    let mut dist: Vec<usize> = vec![INF; n];
    let mut pq: BinaryHeap<(usize, usize)> = BinaryHeap::new();
    // 初期化
    dist[s] = 0;
    pq.push((dist[s], s));
    // 更新
    while let Some((cost, u)) = pq.pop() {
        if dist[u] < cost { continue; }
        for &(v, weight) in &graph[u] {
            if dist[v] > dist[u] + weight {
                dist[v] = dist[u] + weight;
                pq.push((dist[v], v));
            }
        }
    }
    dist
}

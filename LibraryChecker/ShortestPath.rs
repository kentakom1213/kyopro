// https://judge.yosupo.jp/problem/shortest_path

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use std::cmp::Reverse;
use std::collections::BinaryHeap;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// Dijkstra法
fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, s: usize) -> (Vec<usize>, Vec<usize>) {
    let n: usize = graph.len();
    let mut prev: Vec<usize> = vec![INF; n];
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
                prev[v] = u;
                dist[v] = dist[u] + weight;
                pq.push(Reverse((dist[v], v)));
            }
        }
    }
    (prev, dist)
}

// main
fn main() {
    let (N, M, s, t) = get!(usize, usize, usize, usize);

    let mut G = vec![vec![]; N];

    for _ in 0..M {
        let (u, v, w) = get!(usize, usize, usize);
        G[u].push((v, w));
    }

    let (prev, dist) = dijkstra(&G, s);
    let ans = dist[t];

    if ans == INF {
        println!("-1");
    } else {
        // 経路復元
        let mut cur = t;
        let mut path = vec![t];
        while cur != s {
            cur = prev[cur];
            path.push(cur);
        }
        path.reverse();

        // 経路の表示
        let len = path.len() - 1;
        println!("{} {}", ans, len);
        for i in 0..len {
            println!("{} {}", path[i], path[i + 1]);
        }
    }
}

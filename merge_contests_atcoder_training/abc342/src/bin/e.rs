#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use num::traits;
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

macro_rules! debug2D {
    ( $array:expr ) => {{
        #![cfg(debug_assertions)]
        eprintln!("{}: ", stringify!($array));
        for row in &$array {
            eprintln!("{:?}", row);
        }
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: isize = 1001001001001001001;
const UNREACHABLE: &str = "Unreachable";

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        trains: [(isize, isize, isize, isize, Usize1, Usize1); M]
    }

    let mut G = vec![vec![]; N];

    // グラフの構築
    for &(l, d, k, c, a, b) in &trains {
        G[b].push((a, (l, d, k, c)));
    }

    debug!(G);

    // ダイクストラ的なことをする
    let mut latest = vec![-INF; N];
    latest[N - 1] = INF;
    let mut pq = BinaryHeap::from([(INF, N - 1)]);

    // arrivalに間に合う最も遅い電車（なければ-INF）
    let latest_train = |(l, d, k, c), arrival| -> isize {
        if l + c > arrival {
            return -INF;
        }
        let mut ok = -1;
        let mut ng = k;
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            if l + d * mid + c <= arrival {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        l + d * ok
    };

    while let Some((cur, u)) = pq.pop() {
        // 更新できない
        if latest[u] > cur {
            continue;
        }
        // 緩和
        for &(v, info) in &G[u] {
            // 電車の存在判定
            let last = latest_train(info, latest[u]);

            debug!(info, latest[u], last);

            // 更新できる
            if last > latest[v] {
                latest[v] = last;
                pq.push((last, v));
            }
        }
    }

    // 答え
    for &t in &latest[..N - 1] {
        if t == -INF {
            println!("{}", UNREACHABLE);
        } else {
            println!("{t}");
        }
    }
}

mod dijkstra {
    //! ダイクストラ法
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
}

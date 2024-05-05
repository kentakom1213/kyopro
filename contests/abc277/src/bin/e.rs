// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use amplify::confinement::Collection;
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

use std::collections::BinaryHeap;
use std::{cmp::Reverse, collections::VecDeque};

fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
        edges: [(Usize1, Usize1, usize); M],
        S: [Usize1; K],
    }

    // スイッチが存在するか
    let mut has_switch = vec![false; N];
    for &s in &S {
        has_switch[s] = true;
    }

    // グラフの構築
    let mut graph = vec![vec![]; N];
    for &(u, v, a) in &edges {
        graph[u].push((v, a));
        graph[v].push((u, a));
    }

    // dist[u][s] := スイッチが(s?押されていない:押されている)状態の頂点v到達するまでの移動回数の最小値
    let mut dist = vec![[INF; 2]; N];
    dist[0][0] = 0;
    // deq := [(現在の頂点, スイッチが押されているか)]
    let mut deq = VecDeque::new();
    deq.push_back((0, 0));

    while let Some((u, s)) = deq.pop_front() {
        for &(v, a) in &graph[u] {
            // 通行可能な場合（コスト1の辺）
            if s ^ a == 1 {
                if dist[v][s] == INF {
                    dist[v][s] = dist[u][s] + 1;
                    deq.push_back((v, s));
                }
            }
        }
        // スイッチを押す場合（コスト0の辺）
        if has_switch[u] && dist[u][s^1] == INF {
            dist[u][s ^ 1] = dist[u][s];
            deq.push_front((u, s ^ 1));
        }
    }

    debug!(dist);

    let ans = dist[N - 1][0].min(dist[N - 1][1]);
    if ans < INF {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

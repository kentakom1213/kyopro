//           D - Fennec VS. Snuke
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc067/tasks/arc078_b
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::VecDeque;

// imports
use itertools::Itertools;
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// main
fn main() {
    input! {
        N: usize,
        edges: [(Usize1, Usize1); N - 1],
    }

    // グラフの構築
    let mut G = vec![vec![]; N];
    for &(u, v) in &edges {
        G[u].push(v);
        G[v].push(u);
    }

    // 最初のBFS
    let path = {
        let mut q = VecDeque::new();
        q.push_back(0);
        let mut dist = vec![INF; N];
        dist[0] = 0;
        while let Some(u) = q.pop_front() {
            for &v in &G[u] {
                if dist[v] == INF {
                    dist[v] = dist[u] + 1;
                    q.push_back(v);
                }
            }
        }
        // パスの復元
        let mut cur = N - 1;
        let mut path = vec![N - 1];
        while cur != 0 {
            for &prv in &G[cur] {
                if dist[cur] == dist[prv] + 1 {
                    cur = prv;
                    path.push(cur);
                    break;
                }
            }
        }
        path.reverse();
        path
    };

    debug!(&path);
}

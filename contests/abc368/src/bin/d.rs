#![allow(non_snake_case)]

use std::collections::VecDeque;

use cp_library_rs::debug;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        K: usize,
        AB: [(Usize1, Usize1); N - 1],
        V: [Usize1; K]
    }

    let is_stop = V.iter().fold(vec![false; N], |mut x, &v| {
        x[v] = true;
        x
    });

    // グラフの構築
    let G = AB.iter().fold(vec![vec![]; N], |mut g, &(a, b)| {
        g[a].push(b);
        g[b].push(a);
        g
    });

    // 葉を列挙
    let mut leaves = VecDeque::new();
    let mut degree = vec![0; N];

    for u in 0..N {
        degree[u] = G[u].len();
        if degree[u] == 1 {
            leaves.push_back(u);
        }
    }

    debug!(leaves);

    // 葉からdfs
    let mut removed = vec![false; N];
    let mut remaining = N;

    while let Some(u) = leaves.pop_front() {
        if is_stop[u] {
            continue;
        }

        debug!(u);
        removed[u] = true;
        remaining -= 1;

        // 隣接するノードの次数を下げる
        for &v in &G[u] {
            degree[v] -= 1;
        }

        for &v in &G[u] {
            if removed[v] {
                continue;
            }
            // 葉になったら削除可能
            if degree[v] <= 1 {
                leaves.push_back(v);
            }
        }
    }

    debug!(removed);

    println!("{remaining}");
}

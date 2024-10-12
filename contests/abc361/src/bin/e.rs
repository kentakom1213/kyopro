#![allow(non_snake_case)]

use std::collections::VecDeque;

use cp_library_rs::{debug, utils::consts::INF};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        ABC: [(Usize1, Usize1, usize); N - 1]
    }

    let G = ABC.iter().fold(vec![vec![]; N], |mut g, &(a, b, c)| {
        g[a].push((b, c));
        g[b].push((a, c));
        g
    });

    // 木の直径を求める
    let x = {
        let mut dist = vec![INF; N];
        dist[0] = 0;
        let mut q = VecDeque::from([0]);
        while let Some(u) = q.pop_front() {
            for &(v, c) in &G[u] {
                if dist[v] == INF {
                    dist[v] = dist[u] + c;
                    q.push_back(v);
                }
            }
        }
        // 最も遠い頂点
        dist.iter().enumerate().max_by_key(|&(_, &d)| d).unwrap().0
    };

    let (y, D) = {
        let mut dist = vec![INF; N];
        dist[x] = 0;
        let mut q = VecDeque::from([x]);
        while let Some(u) = q.pop_front() {
            for &(v, c) in &G[u] {
                if dist[v] == INF {
                    dist[v] = dist[u] + c;
                    q.push_back(v);
                }
            }
        }
        // 最も遠い頂点
        dist.into_iter()
            .enumerate()
            .max_by_key(|(_, d)| *d)
            .unwrap()
    };

    debug!(x, y);

    let ans = ABC.iter().map(|&(_, _, c)| c).sum::<usize>() * 2 - D;

    println!("{ans}");
}

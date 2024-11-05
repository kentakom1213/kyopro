#![allow(non_snake_case)]

use std::collections::VecDeque;

use cp_library_rs::{debug, utils::consts::INF};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(Usize1, Usize1); M]
    }

    // BFS
    let G = AB.iter().fold(vec![vec![]; N], |mut g, &(a, b)| {
        g[a].push(b);
        g
    });

    let mut ans = INF;
    let mut prev = vec![INF; N];
    let mut dist = vec![INF; N];
    dist[0] = 0;
    let mut q = VecDeque::from([0]);

    'outer: while let Some(u) = q.pop_front() {
        for &v in &G[u] {
            if dist[v] != INF {
                if v == 0 {
                    ans = dist[u] + 1;
                    break 'outer;
                }
                continue;
            }
            dist[v] = dist[u] + 1;
            prev[v] = u;
            q.push_back(v);
        }
    }

    debug!(prev);
    debug!(dist);

    if ans == INF {
        println!("-1");
    } else {
        println!("{ans}");
    }
}

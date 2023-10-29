// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![warn(clippy::default_numeric_fallback)]

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
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

use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        N: usize,
        A: usize,
        B: usize,
        C: usize,
        D: [[usize; N]; N],
    }

    // ダイクストラ法（社用車）
    let dist1 = {
        const INF: usize = 1001001001001001001;
        let n = N;
        let mut dist: Vec<usize> = vec![INF; n];
        let mut pq: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();
        // 初期化
        dist[0] = 0;
        pq.push(Reverse((dist[0], 0)));
        // 更新
        while let Some(Reverse((cost, u))) = pq.pop() {
            if dist[u] < cost {
                continue;
            }
            for v in 0..N {
                let weight = D[u][v] * A;
                if dist[v] > dist[u] + weight {
                    dist[v] = dist[u] + weight;
                    pq.push(Reverse((dist[v], v)));
                }
            }
        }
        dist
    };

    // ダイクストラ法（電車）
    let distN = {
        const INF: usize = 1001001001001001001;
        let n = N;
        let mut dist: Vec<usize> = vec![INF; n];
        let mut pq: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();
        // 初期化
        dist[n - 1] = 0;
        pq.push(Reverse((dist[n - 1], n - 1)));
        // 更新
        while let Some(Reverse((cost, u))) = pq.pop() {
            if dist[u] < cost {
                continue;
            }
            for v in 0..N {
                let weight = D[v][u] * B + C;
                if dist[v] > dist[u] + weight {
                    dist[v] = dist[u] + weight;
                    pq.push(Reverse((dist[v], v)));
                }
            }
        }
        dist
    };

    debug!(&dist1);
    debug!(&distN);

    let mut ans = INF;

    for i in 0..N {
        ans = ans.min(
            dist1[i] + distN[i]
        );
    }

    println!("{}", ans);
}

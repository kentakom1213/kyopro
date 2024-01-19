// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};
use rustc_hash::FxHashMap;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

fn main() {
    input! {
        N: usize,
        M: usize,
        ABC: [(Usize1, Usize1, usize); M],
    }

    // グラフの構築
    let mut dist = vec![vec![INF; N]; N];

    for &(u, v, c) in &ABC {
        dist[u][v] = c;
        dist[v][u] = c;
    }

    // フロイド・ワーシャル法で最短経路対を求める
    for k in 0..N {
        for i in 0..N {
            for j in 0..N {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
            }
        }
    }

    // 重みの小さい辺から採用していく
    let mut ans = 0;

    for &(u, v, c) in &ABC {
        // この辺以外で最短経路を作れるか判定する
        let mut unused = false;
        for i in 0..N {
            unused |= dist[u][i] + dist[i][v] <= c;
        }
        if unused {
            ans += 1;
        }
    }

    println!("{ans}");
}

const INF: usize = 1001001001001001001;

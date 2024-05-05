// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};
use rustc_hash::FxHashSet;

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
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        M: usize,
        edges: [(Usize1, Usize1); M],
        S: Usize1,
        T: Usize1,
    }

    // グラフの構築
    let G = edges.iter().fold(vec![vec![]; N], |mut g, &(u, v)| {
        g[u].push(v);
        g
    });

    // dist[i][j] := 頂点iに距離mod3 == jで到達するときの最短距離
    let mut dist = vec![[INF; 3]; N];
    dist[S][0] = 0;
    let mut q = BinaryHeap::from([Reverse((0, S))]);

    while let Some(Reverse((d, u))) = q.pop() {
        if dist[u][d % 3] < d {
            continue;
        }
        for &v in &G[u] {
            if dist[v][(d + 1) % 3] > dist[u][d % 3] + 1 {
                dist[v][(d + 1) % 3] = dist[u][d % 3] + 1;
                q.push(Reverse((dist[v][(d + 1) % 3], v)));
            }
        }
    }

    debug2D!(dist);

    if dist[T][0] < INF {
        println!("{}", dist[T][0] / 3);
    } else {
        println!("-1");
    }
}

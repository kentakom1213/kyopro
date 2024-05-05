// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::{collections::BinaryHeap, cmp::Reverse};

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

fn main() {
    input! {
        N: usize,
        edges: [(Usize1, Usize1); N - 1]
    }

    // グラフの構築
    let G = edges.iter().fold(vec![vec![]; N], |mut g, &(u, v)| {
        g[u].push(v);
        g[v].push(u);
        g
    });

    // 重み付けしてBFS
    let mut q = BinaryHeap::from([Reverse(0)]);
    let mut is_visited = vec![false; N];
    is_visited[0] = true;
    let mut ans = vec![];

    while let Some(Reverse(u)) = q.pop() {
        ans.push(u + 1);
        for &v in &G[u] {
            if is_visited[v] {
                continue;
            }
            q.push(Reverse(v));
            is_visited[v] = true;
        }
    }

    println!("{}", ans.iter().join(" "));
}

const INF: usize = 1001001001001001001;

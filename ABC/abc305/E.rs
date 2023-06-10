// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::{cmp::Reverse, collections::VecDeque};

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

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
type Graph = Vec<Vec<usize>>;

// main
fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
        edges: [(Usize1, Usize1); M],
        mut PH: [(Usize1, usize); K],
    }

    // グラフの構築
    let mut G = vec![vec![]; N];
    for &(u, v) in &edges {
        G[u].push(v);
        G[v].push(u);
    }


    // HP補充
    let mut add = vec![0; N];
    for &(p, h) in &PH {
        add[p] += h;
    }

    // BFS（すでに到達した頂点は削除）
    let mut visited = vec![false; N];
    for &(p, h) in &PH {
        visited[p] = true;
        let mut q = VecDeque::new();
        q.push_back((p, h));
        while let Some((u, d)) = q.pop_front() {
            for &v in &G[u] {
                if !visited[v] && d + add[v] > 0 {
                    visited[v] = true;
                    q.push_back((v, d - 1 + add[v]));
                }
            }
        }
    }

    // 出力
    let mut ans = vec![];
    for (i, &is_ok) in visited.iter().enumerate() {
        if is_ok {
            ans.push(i + 1);
        }
    }

    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}

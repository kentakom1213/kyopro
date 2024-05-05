// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::{VecDeque, BTreeSet};

use im_rc::{HashMap, HashSet};
// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}};

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

fn main() {
    input! {
        N: usize,
        edges: [(Usize1, Usize1); N - 1],
    }

    // 最大次数の頂点を調べる
    let mut graph = vec![vec![]; N];
    for &(u, v) in &edges {
        graph[u].push(v);
        graph[v].push(u);
    }

    // DFSで彩色
    let mut C = 0;
    let mut color = HashMap::new();
    let mut visited = vec![false; N];
    let mut used = vec![0; N];
    visited[0] = true;
    let mut q = VecDeque::new();
    q.push_back(0);

    while let Some(u) = q.pop_front() {
        let mut c = 0;
        C = C.max(graph[u].len());
        for &v in &graph[u] {
            if visited[v] {
                continue;
            }
            c += 1;
            if used[u] == c {
                c += 1;
            }
            // 使われていない色を調べる
            color.insert((u, v), c);
            color.insert((v, u), c);
            used[v] = c;
            visited[v] = true;
            q.push_back(v);
        }
        debug!(&used);
        debug!(&color);
    }

    // 出力
    println!("{}", C);
    for edge in &edges {
        println!("{}", color[edge]);
    }
}

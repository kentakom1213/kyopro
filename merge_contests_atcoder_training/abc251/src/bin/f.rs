#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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

use std::collections::VecDeque;

use proconio::{
    input,
    marker::{Bytes, Chars, Usize1}, fastout,
};

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        UV: [(Usize1, Usize1); M]
    }

    // グラフの構築
    let G = UV.iter().fold(vec![vec![]; N], |mut g, &(u, v)| {
        g[u].push(v);
        g[v].push(u);
        g
    });

    // T_1 の構成
    // DFSで構築する
    {
        let mut visited = vec![false; N];
        visited[0] = true;
        let mut T1: Vec<(usize, usize)> = vec![];
        dfs(0, &G, &mut T1, &mut visited);

        for &(u, v) in &T1 {
            println!("{} {}", u + 1, v + 1);
        }
    }

    // T_2 の構成
    // BFSで構築する
    {
        let mut visited = vec![false; N];
        visited[0] = true;
        let mut T2: Vec<(usize, usize)> = vec![];

        let mut q = VecDeque::from([0]);
        while let Some(u) = q.pop_front() {
            for &v in &G[u] {
                if visited[v] {
                    continue;
                }
                T2.push((u, v));
                visited[v] = true;
                q.push_back(v);
            }
        }

        for &(u, v) in &T2 {
            println!("{} {}", u + 1, v + 1);
        }
    }
}

const INF: usize = 1001001001001001001;

fn dfs(u: usize, G: &Vec<Vec<usize>>, edges: &mut Vec<(usize, usize)>, visited: &mut Vec<bool>) {
    for &v in &G[u] {
        if visited[v] {
            continue;
        }
        edges.push((u, v));
        visited[v] = true;
        dfs(v, G, edges, visited);
    }
}

// https://atcoder.jp/contests/abc266/tasks/abc266_f

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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

// main
fn main() {
    input! {
        N: usize,
        edges: [(Usize1, Usize1); N],
        Q: usize,
        queries: [(Usize1, Usize1); Q],
    }

    let mut G = vec![vec![]; N];
    for &(u, v) in &edges {
        G[u].push(v);
        G[v].push(u);
    }

    // サイクルを見つける
    let on_cycle = {
        let mut on_cycle = INF;
        let mut visited = vec![false; N];
        let mut que = vec![0];
        while let Some(u) = que.pop() {
            for &v in &G[u] {
                // 未到達の場合
                if visited[v] == false {
                    visited[u] = true;
                    que.push(v);
                }
                // 到達済みの場合
                if visited[v] == true {
                    on_cycle = v;
                    break;
                }
            }
        }
        debug!(&visited, on_cycle);
        on_cycle
    };


    // サイクル上の点を列挙
    let state = {
        let mut state = vec![0; N];
        let mut que = vec![on_cycle];
        state
    };
}

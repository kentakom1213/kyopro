// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::{VecDeque, BTreeSet};

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
        M: usize,
    }
    
    // G[..M] := 超頂点, G[M..] := 頂点
    let mut G = vec![vec![]; M];

    for i in 0..N {
        input!{
            a: usize,
            S: [Usize1; a],
        }
        // 超頂点への追加
        for &v in &S {
            G[v].push(M + i);
        }
        G.push(S);
    }

    debug!(&G[..M], &G[M..]);

    // BFS
    let mut q = VecDeque::new();
    q.push_back(0);
    let mut dist = vec![INF; M + N];
    dist[0] = 0;
    
    while let Some(u) = q.pop_front() {
        for &v in &G[u] {
            if dist[v] == INF {
                dist[v] = dist[u] + 1;
                q.push_back(v);
            }
        }
        debug!(&dist);
    }

    if dist[M - 1] == INF {
        println!("-1");
    } else {
        println!("{}", dist[M - 1] / 2 - 1);
    }
}

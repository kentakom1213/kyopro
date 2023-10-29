// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::VecDeque;

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
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

// main
fn main() {
    input! {
        N1: usize,
        N2: usize,
        M: usize,
        edges: [(Usize1, Usize1); M],
    }

    let N = N1 + N2;

    // グラフ
    let mut G = vec![vec![]; N];

    for &(u, v) in &edges {
        G[u].push(v);
        G[v].push(u);
    }

    // 1からBFS
    let mut dist1 = vec![INF; N];
    dist1[0] = 0;
    let mut q = VecDeque::new();
    q.push_back(0);

    while let Some(u) = q.pop_front() {
        for &v in &G[u] {
            if dist1[v] == INF {
                dist1[v] = dist1[u] + 1;
                q.push_back(v);
            }
        }
    }

    debug!(&dist1);

    // NからBFS
    let mut distN = vec![INF; N];
    distN[N-1] = 0;
    let mut q = VecDeque::new();
    q.push_back(N-1);

    while let Some(u) = q.pop_front() {
        for &v in &G[u] {
            if distN[v] == INF {
                distN[v] = distN[u] + 1;
                q.push_back(v);
            }
        }
    }

    debug!(&distN);

    // それぞれのINFを除いた最大値
    let mut d1_max = 0;
    for &d in &dist1 {
        if d < INF {
            d1_max = d1_max.max(d);
        }
    }

    let mut dN_max = 0;
    for &d in &distN {
        if d < INF {
            dN_max = dN_max.max(d);
        }
    }

    let ans = d1_max + dN_max + 1;
    println!("{}", ans);
}

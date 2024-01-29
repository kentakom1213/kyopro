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
    marker::{Bytes, Chars, Usize1},
};

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

    // dist[S][i] := 空文字列εから始めたpathが状態Sで終端がiであるときの長さの最小値
    let mut dist = vec![vec![INF; N]; 1 << N];
    dist[0][0] = 0;

    let mut q = VecDeque::new();
    for i in 0..N {
        dist[1 << i][i] = 1;
        q.push_back((1 << i, i));
    }

    // 空文字からBFS
    while let Some((S, u)) = q.pop_front() {
        for &v in &G[u] {
            if dist[S ^ 1 << v][v] < INF {
                continue;
            }
            dist[S ^ 1 << v][v] = dist[S][u] + 1;
            q.push_back((S ^ 1 << v, v));
        }
    }

    debug2D!(dist);

    // 答え
    let ans = dist.iter().map(|d| *d.iter().min().unwrap()).sum::<usize>();

    println!("{ans}");
}

const INF: usize = 1001001001001001001;

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
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        UV: [(usize, usize); M]
    }

    let mut X = vec![false; N];
    let mut G = vec![vec![]; N];

    for &(u, v) in &UV {
        if u == 0 {
            X[v - 1] = true;
        } else {
            G[u - 1].push(v - 1);
            G[v - 1].push(u - 1);
        }
    }

    debug!(X);
    debug2D!(G);

    // 頂点0からの距離
    let D1 = bfs(0, &G);
    // 頂点Nからの距離
    let DN = bfs(N - 1, &G);

    debug!(D1, DN);

    // テレポーターがある頂点から頂点0までの距離の最小値
    let P1 = (0..N).filter(|&i| X[i]).map(|i| D1[i]).min().unwrap_or(INF);
    // テレポーターがある頂点から頂点Nまでの距離の最小値
    let PN = (0..N).filter(|&i| X[i]).map(|i| DN[i]).min().unwrap_or(INF);

    debug!(P1, PN);

    // 経由しない場合
    let unvia = D1[N - 1];

    for i in 0..N {
        let via = (P1 + 1).min(D1[i]) + (PN + 1).min(DN[i]);
        let res = via.min(unvia);

        if res == INF {
            print!("-1 ");
        } else {
            print!("{res} ");
        }
    }
    println!();
}

const INF: usize = 1001001001001001001;

/// 頂点`u`をスタートとしたときの各頂点への最短経路
fn bfs(start: usize, G: &Vec<Vec<usize>>) -> Vec<usize> {
    let N = G.len();
    let mut dist = vec![INF; N];
    dist[start] = 0;
    let mut q = VecDeque::from([start]);

    while let Some(u) = q.pop_front() {
        for &v in &G[u] {
            if dist[v] < INF {
                continue;
            }
            dist[v] = dist[u] + 1;
            q.push_back(v);
        }
    }

    dist
}

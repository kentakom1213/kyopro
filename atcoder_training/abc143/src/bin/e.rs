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

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        L: usize,
        ABC: [(Usize1, Usize1, usize); M],
        Q: usize,
        ST: [(Usize1, Usize1); Q]
    }

    // グラフの構築
    let mut dist = vec![vec![INF; N]; N];

    for &(a, b, c) in &ABC {
        dist[a][b] = c;
        dist[b][a] = c;
    }

    // フロイド・ワーシャル法
    for k in 0..N {
        for i in 0..N {
            for j in 0..N {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
            }
        }
    }

    debug2D!(dist);

    // 最短距離がL以下→距離1の辺を張る
    let mut dist2 = vec![vec![INF; N]; N];

    for i in 0..N {
        for j in 0..N {
            if dist[i][j] <= L {
                dist2[i][j] = 1;
            }
        }
    }

    // 2回めのフロイド・ワーシャル法
    for k in 0..N {
        for i in 0..N {
            for j in 0..N {
                dist2[i][j] = dist2[i][j].min(dist2[i][k] + dist2[k][j]);
            }
        }
    }

    debug2D!(dist2);

    // クエリに答える
    for &(s, t) in &ST {
        if dist2[s][t] == INF {
            println!("-1");
        } else {
            println!("{}", dist2[s][t] - 1);
        }
    }
}

const INF: usize = 1001001001001001001;

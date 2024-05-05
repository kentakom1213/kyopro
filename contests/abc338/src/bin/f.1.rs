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

use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

fn main() {
    input! {
        N: usize,
        M: usize,
        UVW: [(Usize1, Usize1, isize); M]
    }

    // グラフの構築
    let G = UVW.iter().fold(vec![vec![INF; N]; N], |mut g, &(u, v, w)| {
        g[u][v] = w;
        g[v][u] = w;
        g
    });

    // フロイド・ワーシャル法
    let dist = {
        let mut dp = G.clone();
        for k in 0..N {
            for i in 0..N {
                for j in 0..N {
                    dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j]);
                }
            }
        }
        dp
    };

    debug2D!(dist);

    // bitDP
    let mut dp = vec![vec![INF; N]; 1 << N];
    dp[0].fill(0);

    for S in 1..1 << N {
        for u in 0..N {
            if S >> u & 1 == 0 {
                continue;
            }
            for v in 0..N {
                if S >> v & 1 == 1 {
                    continue;
                }
                dp[S | 1 << v][v] = dp[S | 1 << v][v].min(dp[S][u] + dist[u][v]);
            }
        }
    }

    debug2D!(dp);

    let ans = *dp[(1 << N) - 1].iter().min().unwrap();

    if ans == INF {
        println!("-1");
    } else {
        println!("{ans}");
    }
}

const INF: isize = 1001001001001001001;

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(Usize1, Usize1); M]
    }

    let G = AB.iter().fold(vec![0_usize; N], |mut g, &(a, b)| {
        g[a] |= 1 << b;
        g[b] |= 1 << a;
        g
    });

    let mut dp = vec![0xff; 1 << N];
    dp[0] = 1;

    // 完全グラフの判定
    for i in 0..N {
        for j in 0..1 << N {
            // 完全グラフのコストは1
            if dp[j] == 1 && (G[i] & j) == j {
                dp[j | 1 << i] = 1;
            }
        }
    }

    // スコアの計算
    for i in 0..1 << N {
        // iの部分集合をすべて調べる
        let mut j = i;
        while j > 0 {
            if dp[i] > dp[j] + dp[i ^ j] {
                dp[i] = dp[j] + dp[i ^ j];
            }
            j -= 1;
            j &= i;
        }
    }

    println!("{}", dp.last().unwrap());
}

const INF: usize = 1001001001001001001;

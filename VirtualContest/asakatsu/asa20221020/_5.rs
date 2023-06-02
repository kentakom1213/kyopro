// https://atcoder.jp/contests/abc034/tasks/abc034_d

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

// imports
use std::collections::{HashMap, BTreeMap, VecDeque};

// input macro
// [Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
macro_rules! get {
    ($t:ty) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($($t:ty),*) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
    ($t:ty ; $n:expr) => {
        (0..$n).map(|_|
            get!($t)
        ).collect::<Vec<_>>()
    };
    ($($t:ty),* ; $n:expr) => {
        (0..$n).map(|_|
            get!($($t),*)
        ).collect::<Vec<_>>()
    };
    ($t:ty ;;) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    ($t:ty ;; $n:expr) => {
        (0..$n).map(|_|
            get!($t ;;)
        ).collect::<Vec<_>>()
    };
}

/*
 * ## 方針
 * - N <= 1000 だからdpっぽい？
 * 
 * ## 更新
 * dp[i][j] := i個までの容器でj個使ったときの濃度の最大値
 * dp[i+1][j+1] = 
 */

// solve
fn main() {
    let (N, K) = get!(usize, usize);
    let wp = get!(f64, f64; N);

    let mut vol = vec![vec![0.0; K+1]; N+1];  // 容量を管理
    let mut dp = vec![vec![0.0; K+1]; N+1];  // 濃度を管理

    for i in 0..N {
        for j in 0..K {
            if dp[i][j] > dp[i+1][j] {
                dp[i+1][j] = dp[i][j];
                vol[i+1][j] = vol[i][j];
            }
            // 新しい濃度
            let new_vol = vol[i][j] + wp[i].0;
            let new_p = (vol[i][j] * dp[i][j] + wp[i].0 * wp[i].1) / new_vol;
            if new_p > dp[i+1][j+1] {
                dp[i+1][j+1] = new_p;
                vol[i+1][j+1] = new_vol;
            }
        }
    }

    // println!("{:?}", dp);
    // println!("{:?}", vol);
    println!("{}", dp[N][K]);
}


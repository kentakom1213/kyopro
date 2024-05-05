//            E - Queen on Grid            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc183/tasks/abc183_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// input macro
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const MOVE: [(usize, usize); 3] = [(0, 1), (1, 0), (1, 1)];

/// ## 方針
/// - 累積和でdpを高速化
fn main() {
    let (H, W) = get!(usize, usize);
    let S: Vec<Vec<char>> = get!(String; H)
        .iter()
        .map(|s|
            s.chars().collect()
        )
        .collect();
    
    // dp[i][j] := マス(i,j)における移動の数
    let mut dp = vec![vec![0; W]; H];
    dp[0][0] = 1;

    // 累積和配列
    let mut X = vec![vec![0; W]; H];
    let mut Y = vec![vec![0; W]; H];
    let mut Z = vec![vec![0; W]; H];

    // 累積和を使って高速化
    for i in 0..H {
        for j in 0..W {
            // 累積和配列を更新 && dp配列を更新
            if i > 0 && S[i-1][j] == '.' {
                X[i][j] = (X[i-1][j] + dp[i-1][j]) % MOD1;
                dp[i][j] += X[i][j];
                dp[i][j] %= MOD1;
            }
            if j > 0 && S[i][j-1] == '.' {
                Y[i][j] = (Y[i][j-1] + dp[i][j-1]) % MOD1;
                dp[i][j] += Y[i][j];
                dp[i][j] %= MOD1;
            }
            if i > 0 && j > 0 && S[i-1][j-1] == '.' {
                Z[i][j] = (Z[i-1][j-1] + dp[i-1][j-1]) % MOD1;
                dp[i][j] += Z[i][j];
                dp[i][j] %= MOD1;
            }
        }
    }

    println!("{}", dp[H-1][W-1]);
}

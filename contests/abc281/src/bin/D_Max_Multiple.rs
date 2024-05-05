//             D - Max Multiple            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc281/tasks/abc281_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
// https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e
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
    ($($t:ty);*) => {
        (
            $(get!($t),)*
        )
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

/// `a < b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmax {
    ( $a:expr, $b:expr ) => {{
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// solve
fn main() {
    let (N, K, D) = get!(usize, usize, usize);
    let A = get!(usize;;);

    // dp[i][j] := i個目までからj個使って合計(mod D)をkにした時の最大値
    let mut dp = vec![vec![vec![None; 111]; 111]; 111];
    dp[0][0][0] = Some(0);

    for i in 0..N {
        for j in 0..=K {
            for k in 0..D {
                if let Some(cur) = dp[i][j][k] {
                    // A[i]を使わない
                    chmax!(
                        dp[i+1][j][k],
                        dp[i][j][k]
                    );

                    // A[i]を使う
                    chmax!(
                        dp[i+1][j+1][(k + A[i]) % D],
                        Some(cur + A[i])
                    );
                }
            }
        }
    }

    // あまりが0になる場合の最大値を探索
    if let Some(ans) = dp[N][K][0] {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

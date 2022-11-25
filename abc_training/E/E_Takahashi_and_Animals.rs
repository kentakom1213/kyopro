//        E - Takahashi and Animals        
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc251/tasks/abc251_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

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
    // ($($t:ty);*) => {
    //     (
    //         $(get!($t),)*
    //     )
    // };
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
const SIZE: usize = 303030;

// solve
fn main() {
    let N = get!(usize);
    let A = get!(usize;;);

    // A1を使うときの費用の合計の最小値
    // dp1[i][j] := 動物iまでに餌を与えたときの費用の合計の最小値（j=0:行動をしたとき、j=1:行動をしていないとき）
    let mut dp1 = vec![vec![INF; 2]; SIZE];
    dp1[0][1] = A[0];
    for i in 1..N {
        // 行動iをするとき
        dp1[i][1] = dp1[i][1].min(dp1[i-1][0] + A[i]);
        dp1[i][1] = dp1[i][1].min(dp1[i-1][1] + A[i]);

        // 行動iをしないとき
        dp1[i][0] = dp1[i][0].min(dp1[i-1][1]);
    }
    let ans1 = dp1[N-1][0].min(dp1[N-1][1]);

    // A1を使わないときの費用の合計の最小値
    let mut dp2 = vec![vec![INF; 2]; SIZE];
    dp2[0][0] = 0;
    dp2[0][1] = INF;
    for i in 1..N {
        // 行動iをするとき
        dp2[i][1] = dp2[i][1].min(dp2[i-1][0] + A[i]);
        dp2[i][1] = dp2[i][1].min(dp2[i-1][1] + A[i]);

        // 行動iをしないとき
        dp2[i][0] = dp2[i][0].min(dp2[i-1][1]);
    }
    let ans2 = dp2[N-1][1];

    // 答え
    let ans = ans1.min(ans2);
    println!("{}", ans);
}

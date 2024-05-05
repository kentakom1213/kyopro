//          D - AtCoder Express 2          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc106/tasks/abc106_d
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

/// ## 方針
/// 二次元累積和
fn main() {
    let (N, M, Q) = get!(usize, usize, usize);
    let span = get!(usize, usize; M);
    let qs = get!(usize, usize; Q);

    let mut LR = vec![vec![0; N+1]; N+1];
    for &(l, r) in &span {
        LR[l][r] += 1;
    }

    // 二次元累積和をとる
    let mut S = vec![vec![0; N+2]; N+2];
    for i in 0..=N {
        for j in 0..=N {
            S[i+1][j+1] = S[i+1][j] + S[i][j+1] - S[i][j] + LR[i][j];
        }
    }

    for &(l, mut r) in &qs {
        r += 1;
        let ans = S[r][r] - S[r][l] - S[l][r] + S[l][l];
        println!("{}", ans);
    }
}

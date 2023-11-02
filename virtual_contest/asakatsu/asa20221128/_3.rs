// https://atcoder.jp/contests/abc222/tasks/abc222_d

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
const MAX: usize = 5000;

// solve
fn main() {
    let N = get!(usize);
    let A = get!(usize;;);
    let B = get!(usize;;);

    // dp
    let mut dp = vec![vec![0_usize; MAX]; MAX];
    // 配列の初期化
    for v in A[0]..=B[0] {
        dp[0][v] = 1;
    }

    // 更新
    for i in 1..N {
        // 一つ前の累積和
        let mut s = vec![0; MAX];
        for j in 0..MAX {
            s[j] = (if j > 0 {s[j-1]} else {0} + dp[i-1][j]) % MOD9;
        }

        // テーブル更新
        for j in A[i]..=B[i] {
            dp[i][j] = s[j];
        }
    }

    let mut ans = 0;
    for i in 0..MAX {
        ans = (ans + dp[N-1][i]) % MOD9;
    }
    println!("{}", ans);
}

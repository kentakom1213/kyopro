//             E - Critical Hit            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc280/tasks/abc280_e
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

// 余りをとる累乗
fn powmod(mut a: usize, mut b: usize, m: usize) -> usize {
    let mut res = 1;
    while b > 0 {
        if b & 1 == 1 {
            res = (res * a) % m;
        }
        a = (a * a) % m;
        b >>= 1;
    }
    res
}

/// ## 方針
/// - 期待値DP
/// - 配るDP
fn main() {
    let (N, P) = get!(usize, usize);

    let inv100 = powmod(100, MOD9-2, MOD9);
    let critical = P * inv100 % MOD9;
    let normal = (MOD9 + 100 - P) % MOD9 * inv100 % MOD9;

    // dp[i] := モンスターに与えたダメージの累計が`i`である時の攻撃回数の期待値
    let mut dp = vec![0; N+1];
    for i in (1..=N).rev() {
        dp[i-1] += (dp[i] + 1) * normal % MOD9;
        dp[i-1] %= MOD9;

        if i > 2 {
            dp[i-2] += (dp[i] + 1) * critical % MOD9;
            dp[i-2] %= MOD9;
        }
    }
    let ans = (dp[1] + 1) % MOD9;
    println!("{}", ans);
}

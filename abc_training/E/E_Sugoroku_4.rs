//              E - Sugoroku 4             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc275/tasks/abc275_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// imports
use std::collections::{HashMap, BTreeMap, VecDeque, BinaryHeap};
use std::cmp::Reverse;

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

// static vales
static MOD1: usize = 1_000_000_007;
static MOD9: usize = 998_244_353;
static INF: usize = 1001001001001001001;

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

// solve
fn main() {
    let (N, M, K) = get!(usize, usize, usize);

    // 逆元の前計算
    let invM = powmod(M, MOD9-2, MOD9);

    // dp[i][j] := iターン目にマスjにいる確率
    let mut dp = vec![vec![0; 1010]; 1010];
    dp[0][0] = 1_usize;

    // 更新
    for i in 0..K {
        for j in 0..N {
            for k in 1..=M {
                let nxt = if j+k <= N {
                    j+k
                } else {
                    2*N - j - k  // j - (k - 2 * (N - j))
                };
                dp[i+1][nxt] += dp[i][j] * invM;
                dp[i+1][nxt] %= MOD9;
            }
        }
        dp[i+1][N] += dp[i][N];
        dp[i+1][N] %= MOD9;
    }

    println!("{}", dp[K][N]);
}

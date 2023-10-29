//            D - Digits Parade            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc135/tasks/abc135_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

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


// solve
fn main() {
    let S = get!(String);
    let digits = S.chars().map(|s| s.to_digit(10).unwrap_or(10) as usize).collect::<Vec<usize>>();
    let N = S.len();  // 桁数

    const MOD: usize = 1_000_000_007;

    // 桁dp
    // dp[i][j] := 数Sの桁数iまでを13で割ったあまりがjになる個数
    let mut dp = vec![vec![0; 13]; N+1];
    dp[0][0] = 1;

    for i in 0..N {
        let c = digits[i];
        for j in 0..10 {
            // cが?ではなく、jでもないときに無視
            if c != 10 && c != j {
                continue;
            }
            // c == '?' or c == j
            for ki in 0..13 {
                dp[i+1][(ki * 10 + j) % 13] += dp[i][ki];
            }
        }
        for j in 0..13 {
            dp[i+1][j] %= MOD;
        }
    }

    let ans = dp[N][5];
    println!("{}", ans);
}


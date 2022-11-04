//          E - Come Back Quickly          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc191/tasks/abc191_e
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
            ($(iter.next().unwrap().parse::<$t>().unwrap(),)*)
        }
    };
    ($t:ty ; $n:expr) => {
        (0..$n).map(|_| get!($t)).collect::<Vec<_>>()
    };
    ($($t:ty),* ; $n:expr) => {
        (0..$n).map(|_| get!($($t),*)).collect::<Vec<_>>()
    };
    ($t:ty ;;) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace().map(|t| t.parse::<$t>().unwrap()).collect::<Vec<_>>()
        }
    };
    ($t:ty ;; $n:expr) => {
        (0..$n).map(|_| get!($t ;;)).collect::<Vec<_>>()
    };
}

// static vales
static MOD1: usize = 1_000_000_007;
static MOD9: usize = 998_244_353;
static INF: usize = 1_000_000_000_000_000_000;


// solve
fn main() {
    let (N, M) = get!(usize, usize);
    let mut dp = vec![vec![INF; N]; N];
    for i in 0..M {
        let (a, b, c) = get!(usize, usize, usize);
        dp[a-1][b-1] = dp[a-1][b-1].min(c);
    }

    // ワーシャルフロイド
    for k in 0..N {
        for i in 0..N {
            for j in 0..N {
                dp[i][j] = dp[i][j].min(
                    dp[i][k] + dp[k][j]
                )
            }
        }
    }

    for i in 0..N {
        if dp[i][i] == INF {
            println!("-1");
        } else {
            println!("{}", dp[i][i]);
        }
    }
}


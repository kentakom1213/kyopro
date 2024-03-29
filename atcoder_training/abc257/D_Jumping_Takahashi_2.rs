//         D - Jumping Takahashi 2         
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc257/tasks/abc257_d
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
static INF: usize = 1_000_000_000_000_000_000;

// solve
fn main() {
    let N = get!(usize);
    let jump = get!(isize, isize, usize; N);

    let mut dp = vec![vec![INF; N]; N];  // ジャンプ台(i,j)間の距離
    for i in 0..N {
        for j in i..N {
            if i == j {
                dp[i][j] = 0;
            }

            let (x1, y1, p1) = jump[i];
            let (x2, y2, p2) = jump[j];
            let dist = ((x1 - x2).abs() + (y1 - y2).abs()) as usize;
            dp[i][j] = (dist + p1 - 1) / p1;
            dp[j][i] = (dist + p2 - 1) / p2;
        }
    }

    // それぞれのジャンプ台間の最短経路を見つける
    for k in 0..N {
        for i in 0..N {
            for j in 0..N {
                dp[i][j] = dp[i][j].min(
                    dp[i][k].max(dp[k][j]),
                );
            }
        }
    }

    let mut ans = INF;
    for i in 0..N {
        let mut tmp = 0;
        for j in 0..N {
            tmp = tmp.max(dp[i][j]);
        }
        ans = ans.min(tmp);
    }

    println!("{}", ans);
}


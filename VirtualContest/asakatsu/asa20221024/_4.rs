// https://atcoder.jp/contests/abc204/tasks/abc204_d

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
    let N = get!(usize);
    let T = get!(usize;;);

    // 数分割問題
    // 部分和問題としてDPで解く
    const MAX_T: usize = 50505;
    let mut dp = vec![vec![0; MAX_T]; 101];
    dp[0][0] = 1;

    for i in 0..N {
        for j in 0..MAX_T {
            if dp[i][j] == 1 {
                // 使わない
                dp[i+1][j] = dp[i][j];
                // 使う
                if j + T[i] < MAX_T {
                    dp[i+1][j+T[i]] = 1;
                }
            }
        }
    }

    // 最もsum(T)/2に近い値を求める
    let sum = T.iter().fold(0, |acc,x| acc+x);
    let mut div2 = 0;
    for i in 0..=(sum / 2) {
        if dp[N][i] == 1 {
            div2 = i;
        }
    }

    println!("{}", sum - div2);
}


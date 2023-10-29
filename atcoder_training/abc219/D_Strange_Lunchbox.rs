//           D - Strange Lunchbox          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc219/tasks/abc219_d
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

/// `a > b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmin {
    ( $a:expr, $b:expr ) => {{
        if $a > $b {
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
    let N = get!(usize);
    let (X, Y) = get!(usize, usize);
    
    // dp[i][j][k] := i個目までの弁当を見たとき、
    //                  - j個以上のたこ焼き
    //                  - k個以上のたい焼き
    //                を食べれるときに買う最小の弁当の個数
    let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![INF; 333]; 333]; 333];
    dp[0][0][0] = 0;

    // dp更新
    for i in 0..N {
        let (a, b) = get!(usize, usize);

        for j in 0..=X {
            for k in 0..=Y {
                // 弁当を買わない
                chmin!(
                    dp[i+1][j][k],
                    dp[i][j][k]
                );

                // 弁当を買う
                chmin!(
                    dp[i+1][X.min(j+a)][Y.min(k+b)],
                    dp[i][j][k] + 1
                );
            }
        }
    }

    if dp[N][X][Y] < INF {
        println!("{}", dp[N][X][Y]);
    } else {
        println!("-1");
    }
}

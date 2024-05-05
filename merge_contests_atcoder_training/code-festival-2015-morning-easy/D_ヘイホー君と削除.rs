//               D - ヘイホー君と削除              
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/code-festival-2015-morning-easy/tasks/cf_2015_morning_easy_d
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

// solve
fn main() {
    let N = get!(usize);
    let S: Vec<char> = get!(String).chars().collect();
    
    let mut longest_lcs = 0;
    for i in 0..N {
        let left = &S[..i];
        let right = &S[i..];

        chmax!(longest_lcs, LCS(left, right));
    }

    let ans = N - longest_lcs * 2;
    println!("{}", ans);
}

/// ## LCS
/// 最長共通部分列を得る
/// 計算量：O(NM)
fn LCS<T: std::cmp::PartialEq> (A: &[T], B: &[T]) -> usize {
    let (la, lb) = (A.len(), B.len());
    let mut dp = vec![vec![0; lb+1]; la+1];

    for (i, a) in A.iter().enumerate() {
        for (j, b) in B.iter().enumerate() {
            if a == b {
                chmax!(dp[i+1][j+1], dp[i][j] + 1);
            }
            chmax!(dp[i+1][j+1], dp[i+1][j]);
            chmax!(dp[i+1][j+1], dp[i][j+1]);
        }
    }

    dp[la][lb]
}

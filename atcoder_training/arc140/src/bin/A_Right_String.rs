//             A - Right String            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc140/tasks/arc140_a
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

// solve
fn main() {
    let (N, K) = get!(usize, usize);
    let S = get!(String).chars().collect::<Vec<char>>();

    // 周期をiに一致させることは可能か
    for i in 1..=N/2 {
        if N % i != 0 { continue; }
        if can(&S, i, K) {
            println!("{}", i);
            return;
        }
    }

    // できない場合
    println!("{}", N);
}

/// 文字列SのK箇所を編集して周期iにすることは可能か判定
fn can(S: &[char], i: usize, K: usize) -> bool {
    let len = S.len() / i;
    let mut res = 0;
    for j in 0..i {
        let mut cnt = vec![0; 26];
        for k in 0..len {
            let c = ord(S[j+i*k]);
            cnt[c] += 1;
        }
        res += len - cnt.iter().max().unwrap();
    }

    res <= K
}

fn ord(c: char) -> usize {
    let a = 'a' as u32;
    let c = c as u32;
    (c - a) as usize
}

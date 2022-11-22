//             A - Continuous 1            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc150/tasks/arc150_a

// WA
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

fn main() {
    let T = get!(usize);
    for _ in 0..T {
        solve();
    }
}

// クエリの処理: O(N)
fn solve() {
    let (N, K) = get!(usize, usize);
    let S = get!(String);

    let mut acc = vec![0; N];
    let (mut l, mut r) = (INF, 0);
    for (i, c) in S.chars().enumerate() {
        if c == '1' {
            // 左右を更新
            if l == INF {
                l = i;
            }
            r = i;
        }
        if c == '1' || c == '?' {
            acc[i] = if i > 0 {acc[i-1]} else {0} + 1;
        }
    }

    let span = r - l;
    let cont_len = *acc.iter().max().unwrap();

    let is_ok = 
            span == K && cont_len >= K
         || span < K && cont_len <= K;
    
    if is_ok {
        println!("Yes");
    } else {
        println!("No");
    }
}

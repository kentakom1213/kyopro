//           D - 2017-like Number          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc084/tasks/abc084_d

// AC
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

// static vales
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// solve
fn main() {
    let Q = get!(usize);

    // エラトステネスの篩で素数を列挙
    let MAX = 200_000;
    let mut sieve = vec![true; MAX];
    sieve[0] = false; sieve[1] = false;
    for i in 2..MAX {
        for j in 2.. {
            if i*j >= MAX { break; }
            sieve[i*j] = false;
        }
    }

    // `2017_like_num`の個数の累積を求める
    let mut S = vec![0; MAX];
    for i in 0..MAX-1 {
        if sieve[i] && sieve[(i+1)/2] {
            S[i+1] = S[i] + 1;
        } else {
            S[i+1] = S[i];
        }
    }

    // クエリの処理
    for _ in 0..Q {
        let (l, r) = get!(usize, usize);
        let res = S[r+1] - S[l-1];
        println!("{}", res);
    }
}

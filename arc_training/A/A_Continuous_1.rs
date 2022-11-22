//             A - Continuous 1            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc150/tasks/arc150_a

// AC
// ----------------------------------------

/*
# 方針
## 1. 判定問題に帰着
S_i, S_i+1, S_i+2, ... S_i+K-1 を全て1に、それ以外を0にできるか？

## 2. 尺取り法で計算量を落とす
区間の差分を管理
*/

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

// クエリの処理
fn solve() {
    let (N, K) = get!(usize, usize);
    let S = get!(String).chars().collect::<Vec<char>>();
    let M = S.iter().filter(|c| **c == '1').count();  // '1'の数

    // 区間の初期化
    let mut cnt = (0, 0);
    for i in 0..K {
        if S[i] == '1' {
            cnt.1 += 1;
        }
        if S[i] == '0' {
            cnt.0 += 1;
        }
    }
    // 条件を満たす区間をカウント
    let mut res = 0;
    for i in 0..=N-K {
        if cnt == (0, M) {
            res += 1;
        }
        if i < N-K {
            if S[i] == '0' {
                cnt.0 -= 1;
            }
            if S[i] == '1' {
                cnt.1 -= 1;
            }
            if S[i+K] == '0' {
                cnt.0 += 1;
            }
            if S[i+K] == '1' {
                cnt.1 += 1;
            }
        }
    }

    if res == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}

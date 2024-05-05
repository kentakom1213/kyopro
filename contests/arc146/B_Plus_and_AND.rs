//             B - Plus and AND            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc146/tasks/arc146_b
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// # get_msb
/// 最上位ビットを調べる
/// ## 方法
/// 1. 最上位bit以下のbitを全て立てる
/// 2. pop_countをとる
fn get_msb(v: usize) -> usize {
    let mut u = v;
    for i in 0..6 {
        u |= u >> (1<<i);
    }
    if u == 0 {
        0
    } else {
        u.count_ones() as usize
    }
}

/// # set_msb
/// 最上位以下のbitを全て1にセットする。（`O(log n)`）
fn set_msb(v: usize) -> usize {
    let mut u = v;
    for i in 0..6 {
        u |= u >> (1 << i);
    }
    u
}

/// # 方針
/// - 二分探索
/// - M回以内の加算で値xを実現できるか → xを最大化
/// 
/// ```non-rust-program
/// 0100 -> 1010 : 1010 - 0100
/// 1110 -> 0101 : (0101 | 1000) - (1110 & 0111)
///              = 1101 - 0110
/// ```
fn main() {
    let (N, M, K) = get!(usize, usize, usize);
    let A = get!(usize;;);

    // M回以内の加算で値xを実現できるか
    let can = |x: usize| -> bool {
        let mut costs: Vec<usize> = A.iter()
            .map(|&v| {
                let mut need = 0;
                for i in (0..32).rev() {
                    if (x >> i) & 1 == 1 && (v >> i) & 1 == 0 {
                        need = x % (1<<i+1) - v % (1<<i+1);
                        break;
                    }
                }
                need
            })
            .collect();

        costs.sort();
        costs[..K].iter().fold(0, |acc, v| acc + v) <= M
    };

    // 二分探索
    let mut ans: usize = 0;
    for i in (0..32).rev().map(|v| 1 << v) {
        if can(ans | i) {
            ans |= i;
        }
    }

    println!("{}", ans);
}

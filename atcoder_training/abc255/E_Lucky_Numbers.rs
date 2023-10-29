//            E - Lucky Numbers            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc255/tasks/abc255_e
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
    ($t:ty) => {{
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
    }};
    ($($t:ty),*) => {{
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
    }};
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
    ($t:ty ;;) => {{
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
    }};
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

/// ## 方針
/// A1を決めれば、A2..ANは一意に定まる。
/// A1=pと定めたとき、
/// ```python
/// A[i] = S[i] - A[i-1]
/// ```
/// ここで、p=0のときのAiの値をBiとする。
/// ```python
/// A[i] = B[i] + (-1)**i * p
/// ```
/// 
/// ## 解法
/// 1. 全ての(A[i], X[i])について、X[i]がA[i]となるときの
/// pの値を調べる。→連想配列で管理
/// 2. 最終的に最も多かったpの値が答えとなる。
fn main() {
    let (N, M) = get!(usize, usize);
    let S = get!(isize;;);
    let X = get!(isize;;);

    let B = {
        let mut arr = vec![0; N];
        for i in 1..N {
            arr[i] = S[i-1] - arr[i-1];
        }
        arr
    };

    let mut cnt_p = BTreeMap::new();
    for &x in &X {
        for i in 0..N {
            let mut modest_p = x - B[i];
            if i % 2 == 1 { modest_p *= -1 }

            *cnt_p.entry(modest_p).or_insert(0) += 1;
        }
    }

    let ans = cnt_p
        .iter()
        .map(|(k, v)| v)
        .max()
        .unwrap();
    
    println!("{}", ans);
}

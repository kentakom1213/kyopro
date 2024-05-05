//                C - 一次元オセロ               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/code-festival-2015-morning-middle/tasks/cf_2015_morning_hard_a
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

/// ## 解説
/// - https://at274.hatenablog.com/entry/2020/06/04/225650
/// - 二手先まで読む貪欲法
fn main() {
    let N = get!(usize);
    let mut A = get!(usize;;);

    let mut ans = 0;
    let (mut l, mut r) = (0, N);  // 左と右のインデックス

    while (r - l) > 1 {
        let l_cost = 2 * A[l] + A[l+1] + 1;
        let r_cost = 2 * A[r-1] + A[r-2] + 1;

        if l_cost <= r_cost {
            // 左の要素を更新
            A[l+2] += A[l] + A[l+1] + 2;
            l += 2;
            // コストを更新
            ans += l_cost;
        } else {
            // 右の要素を更新
            A[r-3] += A[r-1] + A[r-2] + 2;
            r -= 2;
            // コストを更新
            ans += r_cost;
        }
    }

    println!("{}", ans);
}

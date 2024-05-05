//             D - Online games            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc221/tasks/abc221_d
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

/// # BinarySearch
/// 二分探索の実装
trait BinarySearch<T> {
    fn lower_bound(&self, x: T) -> usize;
    fn upper_bound(&self, x: T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    /// ソート済み配列において、`v`以上の最小のインデックスを取得
    fn lower_bound(&self, v: T) -> usize {
        let mut ng = 1_usize.wrapping_neg();
        let mut ok = self.len();
        while ok.wrapping_sub(ng) > 1 {
            let mid = ng.wrapping_add(ok) / 2;
            if v <= self[mid] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    }

    /// ソート済み配列において、`v`より大きい最小のインデックスを取得
    fn upper_bound(&self, v: T) -> usize {
        let mut ng = 1_usize.wrapping_neg();
        let mut ok = self.len();
        while ok.wrapping_sub(ng) > 1 {
            let mid = ng.wrapping_add(ok) / 2;
            if v < self[mid] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    }
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const ARR_SIZE: usize = 202020;

// imos
fn main() {
    let N = get!(usize);
    let ab: Vec<(usize, usize)> = get!(usize, usize; N)
        .iter()
        .map(|&(a, b)| (a, a+b))
        .collect();

    let mut sorted = BTreeSet::new();
    for &(a, b) in &ab {
        sorted.insert(a);
        sorted.insert(b);
    }
    let comp: Vec<usize> = sorted.into_iter().collect();

    // imos法
    let mut imos = vec![0_usize; ARR_SIZE];
    for &(a, b) in &ab {
        let x = comp.lower_bound(a);
        let y = comp.lower_bound(b);
        imos[x] = imos[x].wrapping_add(1);
        imos[y] = imos[y].wrapping_sub(1);
    }

    // 累積和
    let mut S = vec![0_usize; ARR_SIZE];
    for i in 1..ARR_SIZE {
        S[i] = S[i-1].wrapping_add(imos[i-1]);
    }

    // 加算
    let mut ans = vec![0; ARR_SIZE];
    for (i, &v) in S[1..].iter().enumerate() {
        if i + 1 < comp.len() {
            ans[v] += comp[i+1] - comp[i];
        }
    }

    for &d in &ans[1..=N] {
        print!("{} ", d);
    }
    println!();
}


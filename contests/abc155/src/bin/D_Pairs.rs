//                D - Pairs                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc155/tasks/abc155_d
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

trait BinarySearch<T> {
    fn lower_bound(&self, x: T) -> usize;
    fn upper_bound(&self, x: T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    /// ソート済み配列において、`v`以上の最小のインデックスを取得
    fn lower_bound(&self, v: T) -> usize {
        let mut ng = -1;
        let mut ok = self.len() as isize;
        while (ok - ng) > 1 {
            let mid = ((ng + ok) as usize) / 2;
            if v <= self[mid] {
                ok = mid as isize;
            } else {
                ng = mid as isize;
            }
        }
        ok as usize
    }

    /// ソート済み配列において、`v`より大きい最小のインデックスを取得
    fn upper_bound(&self, v: T) -> usize {
        let mut ng = -1;
        let mut ok = self.len() as isize;
        while (ok - ng) > 1 {
            let mid = ((ng + ok) as usize) / 2;
            if v < self[mid] {
                ok = mid as isize;
            } else {
                ng = mid as isize;
            }
        }
        ok as usize
    }
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

/// ## 参考
/// - https://blog.hamayanhamayan.com/entry/2020/02/16/230926
fn main() {
    let (N, K) = get!(usize, usize);
    let mut A = get!(isize;;);
    A.sort();

}

/// Aから重複しないように2要素を選んだとき、その積が負数になる組合せの数
fn get_minus(A: &[isize], lim: isize) -> usize {
    let mut mi = vec![];
    let mut pl = vec![];
    for &a in A {
        if a < 0 { mi.push(a); }
        if a > 0 { pl.push(a); }
    }

    let mut j = 0;
    let mut res = 0;
    for &m in &mi {
        // m * p <= lim
        while j < pl.len() && lim < m * pl[j] {
            j += 1;
        }
        res += pl.len() - j;
    }
    res
}

/// Aから重複しないように2要素を選んだとき、その積が0になる組合せの数
fn get_zeros(A: &[isize]) -> usize {
    todo!();
}

/// Aから重複しないように2要素を選んだとき、その積が正数になる組合せの数
fn get_plus(A: &[isize]) -> usize {
    todo!();
}

/// limは何番目の数なのか
fn get_count(lim: isize) -> usize {
    todo!();
}

//    D - Yet Another Recursive Function   
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc275/tasks/abc275_d
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
type Memo = HashMap<usize, usize>;
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

/// ## 方針
/// メモ化しつつ再帰関数で解く
fn main() {
    let N = get!(usize);
    let mut memo: Memo = HashMap::new();

    let ans = f(N, &mut memo);
    println!("{}", ans);
}

/// メモ化再帰
fn f(n: usize, memo: &mut Memo) -> usize {
    if n == 0 {
        1
    } else {
        let (a, b) = (n/2, n/3);
        let x = if let Some(&v) = memo.get(&a) {
            v
        } else {
            f(a, memo)
        };

        let y = if let Some(&v) = memo.get(&b) {
            v
        } else {
            f(b, memo)
        };

        let res = x + y;
        memo.insert(n, res);
        res
    }
}

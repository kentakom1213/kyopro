//           E - Sorting Queries           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc217/tasks/abc217_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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

/// # 方針
/// - queueとheapqを併用する
fn main() {
    let Q = get!(usize);

    // データ構造を用意
    let mut que: VecDeque<usize> = VecDeque::new();
    let mut pq: BinaryHeap<Reverse<usize>> = BinaryHeap::new();

    for _ in 0..Q {
        let q = get!(String);
        let parse = q.split_whitespace()
                                 .map(|x| x.parse::<usize>().unwrap())
                                 .collect::<Vec<usize>>();
        
        match &parse[..] {
            [_, x] => {
                que.push_back(*x);
            },
            [2] => {
                if let Some(Reverse(top)) = pq.pop() {
                    println!("{}", top);
                } else {
                    println!("{}", que.pop_front().unwrap());
                }
            },
            [3] => {
                while let Some(top) = que.pop_front() {
                    pq.push(Reverse(top));
                }
            },
            _ => unreachable!(),
        }
    }
}

//           B - Reversible Cards          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc111/tasks/arc111_b
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

// solve
fn main() {
    let N = get!(usize);
    let cards = get!(usize, usize; N);

    let mut map = BTreeMap::new();
    for &(a, b) in &cards {
        *map.entry(a).or_insert(0) += 1;
        *map.entry(b).or_insert(0) += 1;
    }

    // 最終的な種類
    let mut kinds = HashSet::new();
    for &(a, b) in &cards {
        let &cnt_a = map.get(&a).unwrap();
        let &cnt_b = map.get(&b).unwrap();
        if cnt_a < cnt_b {
            kinds.insert(a);
            *map.get_mut(&b).unwrap() -= 1;
        } else {
            kinds.insert(b);
            *map.get_mut(&a).unwrap() -= 1;
        }
    }

    println!("{}", kinds.len());
}

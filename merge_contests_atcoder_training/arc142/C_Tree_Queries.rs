//             C - Tree Queries            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc142/tasks/arc142_c
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

// solve
fn main() {
    let N = get!(usize);
    let mut dist = vec![0; N+1];

    for i in 3..=N {
        println!{"? {} {}", 1, i};
        let d_1_i = get!(usize);

        println!("? {} {}", 2, i);
        let d_2_i = get!(usize);

        dist[i] = d_1_i + d_2_i;  // 距離の和
    }

    let ans = *dist[3..].iter().min().unwrap();

    if ans != 3 {
        println!("! {}", ans);
    } else {
        let (mut a, mut b) = (INF, INF);
        for i in 2..=N {
            if dist[i] == 3 {
                if a == INF {
                    a = i;
                }
                b = i;
            }
        }
        println!("? {} {}", a, b);
        let d_a_b = get!(usize);

        if d_a_b == 1 {
            println!("! 3");
        } else {
            println!("! 1");
        }
    }
}

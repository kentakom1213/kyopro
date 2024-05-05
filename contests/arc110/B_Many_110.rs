//               B - Many 110              
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc110/tasks/arc110_b
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

/// ## 方針
/// 1. `T == "1"`：10^10 * 2
/// 2. `T == "11"
fn main() {
    let N = get!(usize);
    let T = get!(String);

    if T == "1" {
        println!("20000000000");
        return;
    }

    let mut cycle110 = "110".chars().cycle();
    
    if T.starts_with("10") {
        cycle110.next();  // 1, 0, 1, 1, 0, ...
    }
    
    if T.starts_with("0") {
        cycle110.next();  // 1, 0, 1, 1, 0, ...
        cycle110.next();  // 0, 1, 1, 0, 1, ...
    }

    let mut cnt_0 = 0;
    for (t, s) in T.chars().zip(cycle110) {
        if t == s {
            if t == '0' {
                cnt_0 += 1;
            }
        } else {
            println!("0");
            return;
        }
    }

    let mut ans = 10_isize.pow(10) - cnt_0;
    if T.ends_with("0") {
        ans += 1;
    }
    println!("{}", ans);
}

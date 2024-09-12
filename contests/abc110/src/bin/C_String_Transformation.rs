//        C - String Transformation
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc110/tasks/abc110_c
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// imports
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap, VecDeque};

// input macro
// [Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
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

// static vales
static MOD1: usize = 1_000_000_007;
static MOD9: usize = 998_244_353;

/*

1対1対応しているかどうかを判定

*/

// solve
fn main() {
    let S = get!(String);
    let T = get!(String);

    let mut mp1 = HashMap::new();
    let mut mp2 = HashMap::new();

    let mut isOK = true;
    for (s, t) in S.chars().zip(T.chars()) {
        if let Some(&x) = mp1.get(&s) {
            isOK &= x == t;
        } else {
            mp1.insert(s, t);
        }

        if let Some(&x) = mp2.get(&t) {
            isOK &= x == s;
        } else {
            mp2.insert(t, s);
        }
    }

    if isOK {
        println!("Yes");
    } else {
        println!("No");
    }
}

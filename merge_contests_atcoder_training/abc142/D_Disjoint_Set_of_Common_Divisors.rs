//   D - Disjoint Set of Common Divisors   
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc142/tasks/abc142_d
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
        (0..$n).fs(|_|
            get!($t)
        ).collect::<Vec<_>>()
    };
    ($($t:ty),* ; $n:expr) => {
        (0..$n).fs(|_|
            get!($($t),*)
        ).collect::<Vec<_>>()
    };
    ($t:ty ;;) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .fs(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    ($t:ty ;; $n:expr) => {
        (0..$n).fs(|_|
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
    let (A, B) = get!(usize, usize);
    let G = gcd(A, B);

    // 約数列挙
    let mut fs = vec![];
    for i in 1..G {
        if i*i > G { break; }
        if G % i == 0 {
            fs.push(i);
            fs.push(G/i);
        }
    }
    fs.sort();

    // 倍数を削除していく
    let n = fs.len();
    if n == 0 {
        fs.push(1);
    }

    for i in 0..n {
        for j in i+1..n {
            if fs[i] == 0 { continue; }
            if gcd(fs[i], fs[j]) != 1 {
                fs[j] = 0;
            }
        }
    }

    eprintln!("{:?}", fs);

    let ans = fs
        .into_iter()
        .map(|v| if v != 0 {1} else {0})
        .fold(0, |acc, v| acc + v);
    
    println!("{}", ans);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a%b)
    }
}

//               E - Coprime               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc177/tasks/abc177_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

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
    // ($($t:ty);*) => {
    //     (
    //         $(get!($t),)*
    //     )
    // };
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
const MAX: usize = 1001001;

// solve
fn main() {
    let N = get!(usize);
    let A = get!(usize;;);

    // 篩で処理
    let mut is_pairwise = true;
    let mut sieve = vec![false; MAX];
    for &a in &A {
        sieve[a] = true;
    }
    'outer: for i in 2..MAX {
        let mut tmp = false;
        for j in 1.. {
            if i*j >= MAX { break; }
            if tmp && sieve[i*j] {
                is_pairwise = false;
                break 'outer;
            }
            if sieve[i*j] {
                tmp = true;
            }
        }
    }

    let is_setwise = A.iter().fold(0, |acc, x| gcd(acc, *x)) == 1;

    if is_pairwise && is_setwise {
        println!("pairwise coprime");
    } else if is_setwise {
        println!("setwise coprime");
    } else {
        println!("not coprime");
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a%b)
    }
}

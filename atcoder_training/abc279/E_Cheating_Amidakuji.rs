//          E - Cheating Amidakuji         
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc279/tasks/abc279_e
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

macro_rules! swap {
    ($arr:expr, $i:expr, $j:expr) => {
        let tmp = $arr[$i];
        $arr[$i] = $arr[$j];
        $arr[$j] = tmp;
    };
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// solve
fn main() {
    let (N, M) = get!(usize, usize);
    let A = get!(usize;;);

    // k = 1,2,...,M について操作を行った際にそれぞれどこにいくかを求める
    let mut B: Vec<usize> = (0..=N).collect();
    for &a in &A {
        swap!(B, a, a+1);
    }

    // すべての操作を行った時点での場所を格納
    let mut pos = vec![0; N+1];
    for i in 1..=N {
        pos[B[i]] = i;
    }

    // 答えを求める
    let mut B: Vec<usize> = (0..=N).collect();
    for &i in &A {
        if B[i] == 1 {
            println!("{}", pos[B[i+1]]);
        } else if B[i+1] == 1 {
            println!("{}", pos[B[i]]);
        } else {
            println!("{}", pos[1]);
        }
        swap!(B, i, i+1);
    }
}

//                 C - 入れ替え                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc004/tasks/abc004_3
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

/// ## 周期に注目
/// - i=0~4までの5回の操作を行うと
/// "123456" -> "234561"
/// つまり、5回の操作で一つずれることがわかる
fn main() {
    let N = get!(usize);

    let m = N / 5 % 6;  // 何回rotateするか

    let to6 = vec![1, 2, 3, 4, 5, 6];
    let mut rotate = to6[m..].to_vec();
    rotate.extend(&to6[..m]);
    
    // 端数を処理
    let rem = N % 5;
    for i in 0..rem {
        swap!(rotate, i, i+1);
    }

    for &a in &rotate {
        print!("{}", a);
    }
    println!();
}

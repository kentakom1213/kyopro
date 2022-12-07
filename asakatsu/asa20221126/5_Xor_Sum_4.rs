
// https://atcoder.jp/contests/abc147/tasks/abc147_d

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

/// ## 方針
/// - 2進数に直した時の桁ごとに考える
/// - 60 * 3e5 = 1.8e7 ←十分高速
/// 
/// ## 実装
/// `a xor b == 1` になる時、`a != b` であることを利用する
/// 条件を満たす組合せは、桁ごとに(0の個数)x(1の個数)個だけ存在する
fn main() {
    let N = get!(usize);
    let A = get!(usize;;);

    let mut ans = 0;
    for i in 0..60 {
        let mut zero = 0;
        let mut one = 0;
        for &a in &A {
            if (a >> i) & 1 == 1 {
                one += 1;
            } else {
                zero += 1;
            }
        }
        
        let tmp = (1 << i) % MOD1
                    * zero % MOD1
                    * one % MOD1;
        ans = (ans + tmp) % MOD1;
    }

    println!("{}", ans);
}

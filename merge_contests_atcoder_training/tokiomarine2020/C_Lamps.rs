//                C - Lamps                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/tokiomarine2020/tasks/tokiomarine2020_c
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
/// ### 状態1
/// ```non-rust-program
///   1  0  0  1  0
/// ------>
///     <->
///        <->
///        <------->
///              <->
/// --|--|--|--|--|--
///   1  2  3  4  5
/// ```
/// 
/// ---
/// ### 状態2
/// ```non-rust-program
///   1  2  2  1  2
/// ------>
/// ------------>
///  <------------->
///        <------->
///        <---------
/// --|--|--|--|--|--
///   1  2  3  4  5
/// ```
fn main() {
    let (N, K) = get!(usize, usize);
    let mut arr1 = get!(usize;;);
    let mut arr2 = vec![0; N];

    for _ in 0..K {
        // imos法
        let mut imos: Vec<usize> = vec![0; N];
        for (i, &a) in arr1.iter().enumerate() {
            let (l, r) = (i.saturating_sub(a), i+a+1);
            imos[l] = imos[l].wrapping_add(1);
            if r < N {
                imos[r] = imos[r].wrapping_sub(1);
            }
        }

        // 累積和
        let mut arr2: Vec<usize> = vec![0; N];
        for i in 0..N {
            arr2[i] = imos[i];
            if i > 0 {
                arr2[i] = arr2[i].wrapping_add(arr2[i-1]);
            }
        }

        if arr1 == arr2 {
            break;
        }

        // 入れ替える
        arr1 = arr2;
        arr2 = vec![0; N];
    }

    // 出力
    for &a in &arr1 {
        print!("{} ", a);
    }
    println!();
}

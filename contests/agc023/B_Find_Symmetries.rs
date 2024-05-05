//           B - Find Symmetries           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/agc023/tasks/agc023_b
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

/// ## ord
/// `A`を`0`とするascii文字(`A~Za~z`)のインデックスを返す
fn ord(c: char) -> usize {
    let a = 'a' as u32;
    let c = c as u32;
    (c - a) as usize
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

/// ## 方針
/// - 愚直な実装: O(N^4)
/// - 一回の比較をO(N)程度に落としたい
/// 
/// ## 解説
/// - https://img.atcoder.jp/agc023/editorial.pdf
/// - 斜めに並行移動しても結果は同じ
/// - したがって、横にずらして判定をN回行えば十分である
fn main() {
    let N = get!(usize);
    let S: Vec<Vec<usize>> = get!(String; N).iter()
        .map(|s|
            s.chars()
                .map(ord)
                .collect()
        )
        .collect();
    
    // 愚直実装
    let mut ans = 0;
    for k in 0..N {

        let mut is_ok = true;  // 対称行列かどうか

        // 対角成分を比較
        for i in 0..N {
            for j in i+1..N {
                let r1 = (N + i - k) % N;
                let c1 = (N + j) % N;
                let r2 = (N + j - k) % N;
                let c2 = (N + i) % N;
                is_ok &= S[r1][c1] == S[r2][c2];
            }
        }

        if is_ok { ans += N; }
    }

    println!("{}", ans);
}

//                 D - Wall                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc079/tasks/abc079_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

// imports
use std::collections::{HashMap, BTreeMap, VecDeque};

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

/* 
 * ## 実装メモ
 * - ワーシャルフロイド法でi->jの最小コストを探索
 */

// solve
fn main() {
    let (H, W) = get!(usize, usize);
    let mut C = get!(isize ;; 10);
    let A = get!(isize ;; H);

    // ワーシャルフロイド法
    for k in 0..10 {
        for i in 0..10 {
            for j in 0..10 {
                C[i][j] = C[i][j].min(
                    C[i][k] + C[k][j]
                )
            }
        }
    }

    let mut ans = 0;
    for i in 0..H {
        for j in 0..W {
            let wall = if A[i][j] >= 0 { A[i][j] as usize } else { 1 };
            ans += C[wall][1];
        }
    }
    println!("{}", ans);
}

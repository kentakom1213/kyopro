//                 C - Z塗り                 
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc040/tasks/arc040_c
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// imports
use std::collections::{HashMap, BTreeMap, VecDeque, BinaryHeap};
use std::cmp::Reverse;

// input macro
// [Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
macro_rules! get {
    (chars) => {
        get!(String).chars().collect::<Vec<char>>()
    };
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

// static vales
static MOD1: usize = 1_000_000_007;
static MOD9: usize = 998_244_353;
static INF: usize = 1001001001001001001;

// solve
fn main() {
    let N = get!(usize);
    let S = get!(String; N);

    let mut field: Vec<Vec<bool>> = S.iter()
                                     .map(|S| S.chars().map(|c| c=='o').collect())
                                     .collect();

    // 常に右上を縫っていく貪欲法
    let mut ans = 0;
    for i in 0..N {
        for j in (0..N).rev() {
            // falseが見つかった時
            if field[i][j] == false {
                for k in 0..=j {
                    field[i][k] = true;
                }
                if i+1 < N {
                    for k in j..N {
                        field[i+1][k] = true;
                    }
                }
                ans += 1;
                break;
            }
        }
    }
    println!("{}", ans);
}

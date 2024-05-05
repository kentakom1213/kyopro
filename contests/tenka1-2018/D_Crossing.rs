//               D - Crossing              
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/tenka1-2018/tasks/tenka1_2018_d
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
    ($t:ty) => {{
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
    }};
    ($($t:ty),*) => {{
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
    }};
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
    ($t:ty ;;) => {{
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
    }};
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
const NEG1: usize = 1_usize.wrapping_neg();

/// ## 方針
/// N = 10
/// ```non-rust-program
/// Yes
/// 5
/// 4 1 2 3 4
/// 4 1 5 6 7
/// 4 2 5 8 9
/// 4 3 6 8 10
/// 4 4 7 9 10
/// ```
fn main() {
    let N = get!(usize);
    let ok: Vec<usize> = (1..1000).map(|x| x * (x-1) / 2).collect();

    match ok.binary_search(&N) {
        Ok(m) => {
            // 構築
            let mut res = vec![vec![0; m]; m + 1];
            let mut cur = 1;

            for i in 0..=m {
                // 重複部分
                for j in 0..i {
                    res[i][j] = res[j][i-1];
                }
                // 独立部分
                for j in i..m {
                    res[i][j] = cur;
                    cur += 1;
                }
            }

            // 出力
            println!("Yes\n{}", m + 1);
            for row in res {
                print!("{} ", m);
                for v in row {
                    print!("{} ", v);
                }
                println!();
            }
        },
        Err(_) => println!("No"),
    }
}

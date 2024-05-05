//                A - B = C                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc112/tasks/arc112_a
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

// static vales
static MOD1: usize = 1_000_000_007;
static MOD9: usize = 998_244_353;
static INF: usize = 1001001001001001001;


// solve
fn main() {
    let T = get!(usize);
    for _ in 0..T {
        solve();
    }
}

fn skip_sum(x: usize) -> usize {
    // 初項`x%2`、末項`x`、項数`n`の等差数列
    let n = (x + 2) / 2;
    n * (x + x%2) / 2
}

fn solve() {
    let (L, R) = get!(usize, usize);

    let mut ans = 0_usize;
    
    // L <= x <= R && x + x <= R
    ans += if 2*L > R { 0 } else { R/2 - L + 1 };

    if 2*L >= R {
        println!("{}", ans);
        return;
    }

    // L <= x < y <= R && x + y <= R
    let range = R - 2*L;
    ans += skip_sum(range) * 2;

    println!("{}", ans);
}

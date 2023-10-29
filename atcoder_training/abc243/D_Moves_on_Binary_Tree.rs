//         D - Moves on Binary Tree        
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc243/tasks/abc243_d
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
static INF: usize = 1_000_000_000_000_000_000;


// solve
fn main() {
    let (N, X) = get!(usize, usize);
    let S = get!(String);

    let mut mov = vec![];
    for c in S.chars() {
        if !mov.is_empty() {
            let tail = mov[mov.len()-1];
            if (tail == 'L' || tail == 'R') && c == 'U' {
                mov.pop();
            } else {
                mov.push(c);
            }
        } else {
            mov.push(c);
        }
    }

    let ans = mov.iter()
                 .fold(
                    X,
                    |x, &c| {
                        if c == 'U' { x / 2 }
                        else if c == 'L' { x*2 }
                        else { x*2 + 1 }
                    }
                 );
    println!("{}", ans);
}

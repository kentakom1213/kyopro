//                C - March                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc089/tasks/abc089_c
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

fn front(s: &str) -> char {
    s.chars().next().unwrap()
}

fn combi(n: usize, r: usize) -> usize {
    if n == r || r == 0 {
        1
    } else {
        (n - r + 1) * combi(n, r-1) / r
    }
}

// solve
fn main() {
    let N = get!(usize);
    let mut map = HashMap::new();
    get!(String; N)
        .iter()
        .filter(|s| match front(s) {
            'M'|'A'|'R'|'C'|'H' => true,
            _ => false,
        })
        .for_each(|s| {
            *map.entry(front(s)).or_insert(0) += 1;
        });
    
    if map.len() < 3 {
        println!("0");
    } else {
        let len = map.len();
        let vec = map.iter().map(|(_, x)| *x).collect::<Vec<usize>>();

        let mut ans = 0;
        for i in 0..len {
            for j in i+1..len {
                for k in j+1..len {
                    ans += vec[i] * vec[j] * vec[k];
                }
            }
        }
        println!("{}", ans);
    }
}

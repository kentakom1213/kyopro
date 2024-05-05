//                E - Bomber               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc176/tasks/abc176_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// imports
use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};
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
    let (H, W, M) = get!(usize, usize, usize);
    let bomb: HashSet<(usize, usize)> = get!(usize, usize; M).into_iter().collect();
    
    // 行と列についてカウント
    let mut row = HashMap::new();
    let mut col = HashMap::new();
    let (mut rmax_val, mut cmax_val) = (0, 0);
    for &(r, c) in &bomb {
        *row.entry(r).or_insert_with(|| 0) += 1;
        *col.entry(c).or_insert_with(|| 0) += 1;
        rmax_val = rmax_val.max( row[&r] );
        cmax_val = cmax_val.max( col[&c] );
    }

    // 値が最大となるキーのリスト
    let rmax: Vec<usize> = row.iter().filter(|(k, &v)| v == rmax_val).map(|(k, v)| *k).collect();
    let cmax: Vec<usize> = col.iter().filter(|(k, &v)| v == cmax_val).map(|(k, v)| *k).collect();

    // 全探索
    let mut ans = rmax_val + cmax_val - 1;
    for &r in &rmax {
        for &c in &cmax {
            if !bomb.contains(&(r, c)) {
                ans += 1;
                println!("{}", ans);
                return;
            }
        }
    }
    println!("{}", ans);
}

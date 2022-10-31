//                 E - Warp                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc265/tasks/abc265_e
// ----------------------------------------

/*
# 解説
dp[n][x][y] := 
*/

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// imports
use std::collections::{HashSet, BTreeMap, VecDeque, BinaryHeap};
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


// solve
fn main() {
    let (N, M) = get!(usize, usize);
    let A = get!(isize;;);
    let mov = [(A[0], A[1]), (A[2], A[3]), (A[4], A[5])];
    let obs: HashSet<(isize, isize)> = get!(isize, isize; M).into_iter().collect();

    let mut dp = vec![vec![1]];
    for i in 0..N {
        let mut new_dp = vec![vec![0; i*2]; i*2];
        for a in 0..=i {
            for b in 0..=i {
                let i = i as isize;
                let a = a as isize;
                let b = b as isize;

                let x = a*mov[0].0 + b*mov[1].0 + (i-a-b)*mov[2].0;
                let y = a*mov[0].1 + b*mov[1].1 + (i-a-b)*mov[2].1;

                for k in 0..3 {
                    let (dx, dy) = mov[k];
                    if obs.contains(&(x+dx, y+dy)) {
                        new_dp[a as usize + (if k==0 {1} else {0})][b as usize + (if k==1 {1} else {0})] += dp[a as usize][b as usize];
                        new_dp[a as usize + (if k==0 {1} else {0})][b as usize + (if k==1 {1} else {0})] %= MOD9;
                    }
                }
            }
        }
        std::mem::swap(&mut dp, &mut new_dp);
    }

    let mut ans = 0;
    for row in &dp {
        for &v in row {
            ans = (ans + v) % MOD9;
        }
    }
    println!("{}", ans);
}


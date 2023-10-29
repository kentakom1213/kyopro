//                 E - Warp                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc265/tasks/abc265_e
// ----------------------------------------

/*
# 解説
dp[n][x][y] := n回の移動後に(x, y)にいるような移動経路の個数

↑こっちは思いつきそう
*/

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// imports
use std::collections::{BTreeMap, VecDeque, BinaryHeap, BTreeSet};
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
    let obs: BTreeSet<(isize, isize)> = get!(isize, isize; M).into_iter().collect();

    let mut dp = BTreeMap::new();
    dp.insert((0, 0), 1);

    for i in 0..N {
        let mut new_dp = BTreeMap::new();
        for (&(x, y), &val) in &dp {
            for &(dx, dy) in &mov {
                let new_pos = (x+dx, y+dy);
                if !obs.contains(&new_pos) {
                    let &old_val = new_dp.get(&new_pos).unwrap_or(&0);
                    new_dp.insert(new_pos, (old_val + val)%MOD9);
                }
            }
        }
        std::mem::swap(&mut dp, &mut new_dp);
    }

    let mut ans = 0;
    for (_, &val) in &dp {
        ans = (ans + val) % MOD9;
    }

    println!("{}", ans);
}

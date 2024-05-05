//                E - (∀x∀)                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc242/tasks/abc242_e
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
    let T = get!(usize);
    for i in 0..T {
        solve();
    }
}

fn ord(c: char) -> usize {
    let a = 'A' as u32;
    let c = c as u32;
    (c - a) as usize
}

fn solve() {
    let N = get!(usize);
    let S = get!(String);

    let N2 = (N + 1) / 2;
    
    // 26進法として数える
    let mut ans = 0;
    for c in S.chars().take(N2) {
        ans *= 26;
        ans %= MOD9;
        ans += ord(c);
        ans %= MOD9;
    }

    // 前半をもとに生成した回文が含まれるかを判定
    let original = S.chars().collect::<Vec<char>>();
    let target = {
        let mut v = S.chars().collect::<Vec<char>>();
        let (mut l, mut r) = (0, N-1);
        while l < r {
            v[r] = v[l];
            l += 1;
            r -= 1;
        }
        v
    };
    
    if original >= target {
        ans += 1;
        ans %= MOD9;
    }

    println!("{}", ans);
}

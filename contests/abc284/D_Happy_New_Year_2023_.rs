//         D - Happy New Year 2023         
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc284/tasks/abc284_d
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

/// 余りをとる累乗
pub fn powmod(a: usize, b: usize, m: usize) -> usize {
    let (mut a, mut  b, m) = (a as u128, b as u128, m as u128);
    let mut res = 1;
    while b > 0 {
        if b & 1 == 1 {
            res = (res * a) % m;
        }
        a = (a * a) % m;
        b >>= 1;
    }
    res as usize
}

/// ## ミラーラビン素数判定法
/// 参考: https://zenn.dev/kaki_xxx/articles/40a92b43200215
pub fn is_prime_MR(N: usize) -> bool {
    if N <= 2 {
        return N == 2;
    }
    if N % 2 == 0 {
        return false;
    }

    let (mut s, mut d) = (0, N - 1);
    while d % 2 == 0 {
        s += 1;
        d >>= 1;
    }

    // n < 2^64 の場合、以下を調べれば十分
    let A = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];
    for &a in &A {
        if a % N == 0 { break; }
        let mut t = 0;
        let mut x = powmod(a, d, N);
        if x != 1 {
            while t < s {
                if x == N - 1 { break; }
                x = ((x as u128).pow(2) % (N as u128)) as usize;
                t += 1;
            }
            if t == s { return false; }
        }
    }
    true
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// ## pollard_rho
/// ポラード・ロー法を適用し、約数を見つける
fn pollard_rho(N: usize) -> usize {
    if N % 2 == 0 { 
        return 2;
    }
    if is_prime_MR(N) {
        return N;
    }
    let f = |x: usize| -> usize {
        (((x as u128).pow(2) + 1) % N as u128) as usize
    };
    let mut step = 0;
    loop {
        step += 1;
        let mut x = step;
        let mut y = f(x);
        loop {
            let p = gcd(N + y - x, N);
            if p == 0 || p == N { break; }
            if p != 1 { return p; }
            x = f(x);
            y = f(f(y));
        }
    }
}

/// ## factorize
/// ポラード・ロー法による高速素因数分解
/// `O(n^(1/4))`
fn factorize(N: usize) -> Vec<usize> {
    if N == 1 { return vec![]; }
    let p = pollard_rho(N);
    if p == N { return vec![N]; }
    let mut left = factorize(p);
    let mut right = factorize(N / p);
    left.append(&mut right);
    // left.sort();
    left
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// solve
fn main() {
    let T = get!(usize);
    for _ in 0..T {
        solve();
    }
}

/// ## 方針
/// - ミラー・ラビン素数判定法
/// - ポラード・ロー素因数分解法
/// を組み合わせて、高速な素因数分解を行う。
/// ### 参考
/// - [Nyaan's Library](https://nyaannyaan.github.io/library/prime/fast-factorize.hpp.html)
fn solve() {
    let t = get!(usize);
    let ans = factorize(t);
    let (a, b, c) = (ans[0], ans[1], ans[2]);

    if a == b {
        println!("{} {}", a, c);
    } else {
        println!("{} {}", c, a);
    }
}

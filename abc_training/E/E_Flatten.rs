//               E - Flatten               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc152/tasks/abc152_e
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
    ($($t:ty);*) => {
        (
            $(get!($t),)*
        )
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

/// `a < b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmax {
    ( $a:expr, $b:expr ) => {{
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

// constant
type Factors = HashMap<usize, usize>;
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// solve
fn main() {
    let N = get!(usize);
    let A = get!(usize;;);

    let mut lcm: Factors = HashMap::new();
    let facts: Vec<Factors> = A
        .iter()
        .map(|&v| factor(v))
        .collect();

    // 最大公約数（素因数分解の形式を求める）
    for fac in &facts {
        for (&f, &c) in fac {
            chmax!(*lcm.entry(f).or_insert(0), c);
        }
    }

    // 解を求める
    let mut ans = 0;
    for fac in &facts {
        let mut tmp = 1;
        for (&f, &c) in &lcm {
            let v = fac.get(&f).unwrap_or(&0);
            tmp *= powmod(f, c - *v,MOD1);
            tmp %= MOD1;
        }
        ans += tmp;
        ans %= MOD1;
    }

    println!("{}", ans);
}

/// ## 素因数分解
fn factor(mut n: usize) -> HashMap<usize, usize> {
    let mut res = HashMap::new();
    for i in 2..n {
        if i*i > n { break; }
        while n % i == 0 {
            *res.entry(i).or_insert(0) += 1;
            n /= i;
        }
    }
    if n > 1 {
        *res.entry(n).or_insert(0) += 1;
    }
    res
}

// 余りをとる累乗
fn powmod(mut a: usize, mut b: usize, m: usize) -> usize {
    let mut res = 1;
    while b > 0 {
        if b & 1 == 1 {
            res = (res * a) % m;
        }
        a = (a * a) % m;
        b >>= 1;
    }
    res
}

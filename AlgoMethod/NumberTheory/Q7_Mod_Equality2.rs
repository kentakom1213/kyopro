// あまりが等しい(2)
// -----------------
// 問題
// https://algo-method.com/tasks/551
// -----------------

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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// ## 素因数分解
fn factor(mut n: usize) -> HashMap<usize, usize> {
    let mut res = HashMap::new();
    for i in 2..n {
        if i * i > n {
            break;
        }
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

/// ## 方針
/// - すべての要素をA_0からの差分で表す
/// - それらのGCDを素因数分解
fn main() {
    let N = get!(usize);
    let A = get!(isize;;);

    let GCD = A.iter().map(|&a| (a - A[0]).abs() as usize).fold(0, gcd);

    let fact = factor(GCD);

    let mut ans = 1;
    for (_, &v) in &fact {
        ans *= v + 1;
    }

    println!("{}", ans);
}

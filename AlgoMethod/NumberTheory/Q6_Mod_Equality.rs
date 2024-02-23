// あまりが等しい
// -----------------
// 問題
// https://algo-method.com/tasks/549
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

/// ## 解説
/// ```non-rust-program
/// - `A = q1 * x + r`
/// - `B = q2 * x + r`
/// <=> A - B = (q1 - q2) * x
/// ```
fn main() {
    let (A, B) = get!(usize, usize);
    let diff = if A > B { A - B } else { B - A };

    let fact = factor(diff);

    let mut ans = 1;
    for (_, &v) in &fact {
        ans *= v + 1;
    }

    println!("{}", ans);
}

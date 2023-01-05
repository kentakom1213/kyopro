//            D - Patisserie ABC           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc100/tasks/abc100_d
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

const INF: isize = 1001001001001001001;

/// 解説AC
/// - 「和の絶対値」を最大化したい
/// - 3つの要素それぞれについて正負を取れば良いので 2^3 = 8 通りの探索をすれば十分
fn main() {
    let (N, M) = get!(usize, usize);
    let cakes = get!(isize, isize, isize; N);

    // bit演算で高速化
    let mut ans = -1;
    for i in 0..(1 << 3) {
        let mut sums = vec![0; N];
        for j in 0..N {
            let signs = (
                (-1_isize).pow(i>>0 & 1),
                (-1_isize).pow(i>>1 & 1),
                (-1_isize).pow(i>>2 & 1)
            );
            sums[j]
                = signs.0 * cakes[j].0
                + signs.1 * cakes[j].1
                + signs.2 * cakes[j].2;
        }
        sums.sort();
        let tmp = sums[N-M..].iter().fold(0, |acc, v| acc + v);
        chmax!(ans, tmp);
    }

    println!("{}", ans);
}

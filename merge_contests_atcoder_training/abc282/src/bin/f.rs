// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::fmt::format;

// imports
use itertools::Itertools;
use rustc_hash::FxHashMap;
// use proconio::{input, marker::{Chars, Bytes, Usize1}};

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

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

macro_rules! debug2D {
    ( $array:expr ) => {{
        #![cfg(debug_assertions)]
        eprintln!("{}: ", stringify!($array));
        for row in &$array {
            eprintln!("{:?}", row);
        }
    }};
}

fn main() {
    let N = get!(usize);

    let b = 64 - N.leading_zeros() as usize;

    // SparseTableの区間を構築すればよい
    let mut table = vec![vec![]; b];
    let mut ranges = vec![];

    for i in 0..b {
        let mut j = 0;
        while j + (1 << i) <= N {
            let rng = (j, j + (1 << i));
            ranges.push(rng);
            table[i].push(rng);
            j += 1;
        }
    }

    debug2D!(table);
    debug!(ranges);

    // 区間取得用の配列
    let mut logs = vec![0; N + 1];
    for i in 2..=N {
        logs[i] = logs[i >> 1] + 1;
    }

    debug!(logs);

    // 逆転させたmap
    let map = ranges
        .iter()
        .enumerate()
        .fold(FxHashMap::default(), |mut map, (i, &(l, r))| {
            map.insert((l, r), i + 1);
            map
        });

    // 区間の出力
    println!("{}", ranges.len());
    println!(
        "{}",
        ranges
            .iter()
            .map(|&(a, b)| format!("{} {}", a + 1, b))
            .join("\n")
    );

    // クエリ処理
    let Q = get!(usize);

    for _ in 0..Q {
        let (mut l, r) = get!(usize, usize);
        l -= 1;

        let lg = logs[r - l];

        let a = table[lg][l];
        let b = table[lg][r - (1 << lg)];
        debug!(a, b);

        println!("{} {}", map[&a], map[&b]);
    }
}

const INF: usize = 1001001001001001001;

// https://atcoder.jp/contests/abc002/tasks/abc002_4

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
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

/// ## 方針
/// - 最大クリークを求める
/// - bit全探索
fn main() {
    let (N, M) = get!(usize, usize);
    let xy: Vec<_> = get!(usize, usize; M)
        .iter()
        .map(|(x, y)| (x-1, y-1))
        .collect();

    // 連結判定をO(1)にするため、隣接行列を作成
    let G = {
        let mut graph = vec![vec![false; N]; N];
        for &(x, y) in &xy {
            graph[x][y] = true;
            graph[y][x] = true;
        }
        graph
    };

    // bit全探索
    let mut ans = 0;
    for i in 0..(1<<N) {
        let mut group = vec![];
        for j in 0..N {
            if (i >> j) & 1 == 1 {
                group.push(j);
            }
        }
        
        let mut is_comp = true;  // 完全グラフであるか
        let siz = group.len();
        for k in 0..siz {
            for l in k+1..siz {
                let (u, v) = (group[k], group[l]);
                is_comp &= G[u][v];
            }
        }

        if is_comp {
            chmax!(ans, siz);
        }
    }

    println!("{}", ans);
}

//              C - Blue Bird              
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc022/tasks/abc022_c
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// imports
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
    // ($($t:ty);*) => {
    //     (
    //         $(get!($t),)*
    //     )
    // };
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

// solve
fn main() {
    let (N, M) = get!(usize, usize);
    let mut G0 = vec![];
    let mut G = vec![vec![INF; N]; N];
    for i in 0..M {
        let (mut u, mut v, l) = get!(usize, usize, usize);
        u -= 1; v -= 1;
        if u == 0 {
            G0.push((v, l));
        } else if v == 0 {
            G0.push((u, l));
        } else {
            G[u][v] = l;
            G[v][u] = l;
        }
    }

    // 全頂点対最短経路探索
    for c in 0..N {
        for a in 0..N {
            for b in 0..N {
                if G[a][c] + G[c][b] < G[a][b] {
                    G[a][b] = G[a][c] + G[c][b];
                }
            }
        }
    }

    // 頂点0に隣接する頂点の列挙
    let mut ans = INF;
    for (i, &(u, w1)) in G0.iter().enumerate() {
        for &(v, w2) in &G0[i+1..] {
            ans = ans.min(
                w1 + G[u][v] + w2
            );
        }
    }

    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

//            E - Small d and k            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc254/tasks/abc254_e

// AC
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

/// ## 方針
/// - 一回のクエリに高々9回の計算しか行わない
/// - クエリを愚直に処理する
fn main() {
    let (N, M) = get!(usize, usize);
    let mut G = vec![vec![]; N];
    for i in 0..M {
        let (a, b) = get!(usize, usize);
        G[a-1].push(b-1);
        G[b-1].push(a-1);
    }
    
    let Q = get!(usize);
    for i in 0..Q {
        let (x, k) = get!(usize, usize);
        let ans = bfs(x-1, k, &G);
        println!("{}", ans);
    }
}

/// bfsで探索を行う
fn bfs(x: usize, k: usize, graph: &Vec<Vec<usize>>) -> usize {
    let mut q = VecDeque::new();
    q.push_back((x, 0));  // (現在の頂点, `x`からの深さ)
    let mut is_visited = vec![false; graph.len()];
    let mut res = HashSet::new();  // あらかじめ頂点`x`を追加しておく
    
    while let Some((u, d)) = q.pop_front() {
        res.insert(u + 1);
        for &v in &graph[u] {
            if is_visited[v] { continue; }
            if d+1 <= k {
                q.push_back((v, d+1));
            }
            is_visited[v] = true;
        }
    }
    res.iter().fold(0, |acc, x| acc + x)
}

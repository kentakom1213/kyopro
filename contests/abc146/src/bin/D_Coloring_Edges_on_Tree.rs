//        D - Coloring Edges on Tree       
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc146/tasks/abc146_d

// AC
// 難しい
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
    let n = get!(usize);
    let mut graph = vec![vec![]; n];
    let mut edges = vec![];
    for i in 0..n-1 {
        let (mut a, mut b) = get!(usize, usize);
        a -= 1; b -= 1;
        graph[a].push(b);
        graph[b].push(a);
        edges.push((a, b));
    }

    let K = graph.iter().map(|x| x.len()).max().unwrap();
    let mut I = HashMap::new();
    let mut col = vec![0; n];

    // bfsで塗り分け
    let mut visited = vec![false; n];
    let mut q = VecDeque::new();
    visited[0] = true;
    q.push_back(0);
    while let Some(u) = q.pop_back() {
        let mut cur = 1;
        for &v in &graph[u] {
            if visited[v] { continue; }
            if cur == col[u] { cur += 1; }

            col[v] = cur;
            *I.entry((u, v)).or_insert(0) = cur;
            *I.entry((v, u)).or_insert(0) = cur;
            cur += 1;

            visited[v] = true;
            q.push_back(v);
        }
    }

    println!("{}", K);
    for edge in &edges {
        println!("{}", I.get(edge).unwrap());
    }
}

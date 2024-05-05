//        C - Cat Snuke and a Voyage       
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc068/tasks/arc079_a
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

// imports
use std::collections::{HashMap, BTreeMap, VecDeque};

// input macro
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

// solve
fn main() {
    let (N, M) = get!(usize, usize);
    let mut G = vec![vec![]; N];
    for i in 0..M {
        let (a, b) = get!(usize, usize);
        G[a-1].push(b-1);
        G[b-1].push(a-1)
    }

    // bfs
    let mut dist = vec![-1; N];
    dist[0] = 0;

    let mut que = VecDeque::new();
    que.push_back(0);

    while !que.is_empty() {
        let cur = que.pop_front().unwrap();
        for &nxt in &G[cur] {
            if dist[nxt] == -1 {
                dist[nxt] = dist[cur] + 1;
                que.push_back(nxt);
            }
        }
    }

    if 1 <= dist[N-1] && dist[N-1] <= 2 {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}
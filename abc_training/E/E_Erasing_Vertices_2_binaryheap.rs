//          E - Erasing Vertices 2         
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc267/tasks/abc267_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

// imports
use std::collections::{HashMap, BTreeMap, VecDeque, BinaryHeap};
use std::cmp::Reverse;

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
    let A = get!(isize;;);
    let mut G = vec![vec![]; N];
    let mut cost = vec![0; N];
    for _ in 0..M {
        let (mut u, mut v) = get!(usize, usize);
        u -= 1; v -= 1;
        G[u].push(v);
        G[v].push(u);
        cost[u] += A[v];
        cost[v] += A[u];
    }

    let mut heapq = BinaryHeap::new();
    for (i, &c) in cost.iter().enumerate() {
        heapq.push(Reverse((c, i)));
    }

    let mut erased = vec![false; N];
    let mut ans = 0;

    while !heapq.is_empty() {
        let Reverse((c, u)) = heapq.pop().unwrap();
        if erased[u] {
            continue;
        }
        erased[u] = true;
        ans = ans.max(c);
        for &v in &G[u] {
            if !erased[v] {
                cost[v] -= A[u];
                heapq.push(Reverse((cost[v], v)));
            }
        }
    }

    println!("{}", ans);
}


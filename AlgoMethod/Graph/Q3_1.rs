// https://algo-method.com/tasks/414

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

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

static INF: usize = 1001001001001001001;

// solve
fn main() {
    let (N, M) = get!(usize, usize);
    let mut G = vec![vec![]; N];
    for i in 0..M {
        let (a, b) = get!(usize, usize);
        G[a].push(b);
        G[b].push(a);
    }

    // 原点からの距離を求める
    let mut dist = vec![INF; N];
    dist[0] = 0;
    let mut q = VecDeque::new();
    q.push_back(0);
    while let Some(u) = q.pop_front() {
        for &v in &G[u] {
            if dist[v] == INF {
                dist[v] = dist[u] + 1;
                q.push_back(v);
            }
        }
    }

    // 出力
    let mut ans = vec![vec![]; N];
    for (i, &d) in dist.iter().enumerate() {
        if d == INF { continue; }
        ans[d].push(i);
    }
    
    for v in &ans {
        v.into_iter().for_each(|x| print!("{} ", x));
        println!();
    }
}


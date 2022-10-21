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
    let A = get!(usize;;);
    let mut G = vec![vec![]; N];
    for _ in 0..M {
        let (mut u, mut v) = get!(usize, usize);
        u -= 1; v -= 1;
        G[u].push(v);
        G[v].push(u);
    }

    let can = |m: usize| -> bool {
        let mut cost = vec![0; N];
        for u in 0..N {
            for &v in &G[u] {
                cost[u] += A[v];
            }
        }
        let mut is_delete = vec![false; N];
        let mut que = VecDeque::new();
        for i in 0..N {
            if cost[i] <= m {
                que.push_back(i);
            }
        }
        while !que.is_empty() {
            let u = que.pop_front().unwrap();
            is_delete[u] = true;
            for &v in &G[u] {
                if !is_delete[v] {
                    cost[v] = if cost[v] <= A[u] { 0 } else { cost[v] - A[u] };
                    if cost[v] <= m {
                        que.push_back(v);
                    }
                }
            }
        }

        is_delete
            .iter()
            .fold(true, |acc, x| acc & x)
    };

    let mut left = 0;
    let mut right = 1001001001001;
    while (right - left) > 1 {
        let mid = (left + right) / 2;
        if can(mid) {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", right);
}


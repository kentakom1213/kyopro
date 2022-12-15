//                B - ツリーグラフ               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc030/tasks/arc030_2
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

type G = Vec<Vec<usize>>;

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// solve
fn main() {
    let (N, mut X) = get!(usize, usize);
    X -= 1;

    let H = get!(isize;;);
    let graph: G = {
        let mut g = vec![vec![]; N];
        for _ in 0..N-1 {
            let (mut a, mut b) = get!(usize, usize);
            a -= 1; b -= 1;
            g[a].push(b);
            g[b].push(a);
        }
        g
    };

    // 子が宝石を含むかどうかを探索（木dp）
    let mut cnt = vec![-1; N];
    dfs1(INF, X, &mut cnt, &H, &graph);

    // 子が宝石を含むようなパスを作る
    let mut path = vec![];
    dfs2(INF, X, &mut path, &cnt, &graph);

    // println!("{:?}", path);
    println!("{}", path.len());
}

/// ## 木dpを行う
/// - 部分木に含まれる宝石の個数をカウント
fn dfs1(p: usize, u: usize, cnt: &mut [isize], H: &[isize], graph: &G) {
    if graph[u].len() == 1 && p != INF {
        // 葉
        cnt[u] = H[u];
        return;
    }
    let mut jewelry = H[u];
    for &v in &graph[u] {
        if v != p {
            dfs1(u, v, cnt, H, graph);
            jewelry += cnt[v];
        }
    }
    cnt[u] = jewelry;
}

/// ## パスの長さを求める
/// - 部分木に宝石がある場合のみ探索
fn dfs2(p: usize, u: usize, path: &mut Vec<usize>, cnt: &[isize], graph: &G) {
    for &v in &graph[u] {
        if v == p || cnt[v] == 0 { continue; }
        path.push(v+1);
        dfs2(u, v, path, cnt, graph);
        path.push(v+1);
    }
}

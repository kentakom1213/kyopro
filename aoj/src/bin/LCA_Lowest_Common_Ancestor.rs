//       LCA: Lowest Common Ancestor       
// ----------------------------------------
// 問題
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_C&lang=jp
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// solve
fn main() {
    let N = get!(usize);
    let graph: Vec<Vec<usize>> = get!(usize;; N)
        .iter()
        .map(|l|
            l.iter()
             .skip(1)
             .cloned()
             .collect()
        )
        .collect();
    
    let mut depth = vec![INF; N];
    depth[0] = 0;
    dfs(0, &mut depth, &graph);

    // ダブリング配列
    // dp[i][j] := jから根に向かって2^i回進んだときの頂点
    let mut dp = vec![vec![0; N]; 20];
    for u in 0..N {
        for &v in &graph[u] {
            dp[0][v] = u;
        }
    }

    for i in 0..19 {
        for j in 0..N {
            dp[i+1][j] = dp[i][dp[i][j]];
        }
    }

    // 頂点uから根方向にn回進んだときの頂点 -> v
    let f = |u: usize, n: usize| -> usize {
        let mut v = u;
        for i in 0..20 {
            if (n >> i) & 1 == 1 {
                v = dp[i][v];
            }
        }
        v
    };

    // 二分探索（f(u, x) == f(v, x) となる最小のxを求める）
    let q = get!(usize);
    for _ in 0..q {
        let (mut u, mut v) = get!(usize, usize);

        // 常にuの方が深い
        if depth[u] < depth[v] { std::mem::swap(&mut u, &mut v) }

        // LCAまでの距離を同じにする
        for k in 0..20 {
            if ((depth[u] - depth[v]) >> k) & 1 == 1 {
                u = dp[k][u];
            }
        }

        if u == v {
            println!("{}", u);
        } else {
            // 二分探索
            // 大きい方から決めていく
            for k in (0..20).rev() {
                if dp[k][u] != dp[k][v] {
                    u = dp[k][u];
                    v = dp[k][v];
                }
            }
            println!("{}", dp[0][u]);
        }
    }
}

fn dfs(u: usize, depth: &mut Vec<usize>, graph: &Vec<Vec<usize>>) {
    for &v in &graph[u] {
        if depth[v] != INF { continue; }
        depth[v] = depth[u] + 1;
        dfs(v, depth, graph);
    }
}

//                  N - 木                  
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/tdpc/tasks/tdpc_tree
// ----------------------------------------

/*

## 方針
- 木dp
- 辺から書いていく場合であるので、子から総和を取っていく

*/

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

// static vales
static MOD1: usize = 1_000_000_007;
static MOD9: usize = 998_244_353;


// solve
fn main() {
    let N = get!(usize);
    let mut G = vec![vec![]; N];
    for _ in 0..N-1 {
        let (a, b) = get!(usize, usize);
        G[a-1].push(b-1);
        G[b-1].push(a-1);
    }

    let mut used = vec![false; N];
    let mut dp = vec![0; N];

    // 木dp
    dfs(0, &mut used, &mut dp, &G);

    println!("{:?}", dp);
    println!("{}", dp[0]);
}

fn dfs(cur: usize, used: &mut Vec<bool>, dp: &mut Vec<usize>, graph: &Vec<Vec<usize>>) {
    used[cur] = true;  // 完了済みとしてマーク

    for &nxt in &graph[cur] {
        if used[nxt] { continue; }
        dfs(nxt, used, dp, graph);

        dp[cur] += dp[nxt];
        dp[cur] %= MOD1;
    }

    // 初期値を設定
    dp[cur] += 1;
}

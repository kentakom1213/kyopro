//      073 - We Need Both a and b（★5）     
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_bu
// ----------------------------------------

/*

## 方針
- 木DP

dp[pos][今の連結成分にどれがあるか] := 頂点`pos`の部分木を考えたときに何通りあるか

今の連結成分にどれがあるか = {
    0: {'a',},
    1: {'b',},
    2: {'a', 'b'},
}

## 参考
- https://github.com/E869120/kyopro_educational_90/blob/main/sol/073.cpp

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
    let C = get!(char;;);
    let mut G = vec![vec![]; N];
    for _ in 0..N-1 {
        let (a, b) = get!(usize, usize);
        G[a-1].push(b-1);
        G[b-1].push(a-1);
    }

    // dp[pos][今の連結成分にどれがあるか] := 頂点`pos`の部分木を考えたときに何通りあるか
    let mut dp = vec![vec![0; 3]; N];

    // 木dp
    dfs(0, 0, &G, &C, &mut dp);

    // 答え
    println!("{}", dp[0][2]);
}

fn dfs(prev: usize, cur: usize, graph: &Vec<Vec<usize>>, color: &Vec<char>, dp: &mut Vec<Vec<usize>>) {
    let (mut val1, mut val2) = (1, 1);
    for &nxt in &graph[cur] {
        if nxt == prev { continue; }

        // 子について呼び出し
        dfs(cur, nxt, &graph, color, dp);  // ← 既にmutだから宣言しなくても良い？

        if color[cur] == 'a' {
            val1 *= dp[nxt][0] + dp[nxt][2];
            val2 *= dp[nxt][0] + dp[nxt][1] + 2 * dp[nxt][2];
        }
        if color[cur] == 'b' {
            val1 *= dp[nxt][1] + dp[nxt][2];
            val2 *= dp[nxt][0] + dp[nxt][1] + 2 * dp[nxt][2];
        }
        val1 %= MOD1;
        val2 %= MOD1;
    }

    if color[cur] == 'a' {
        dp[cur][0] = val1;
        dp[cur][2] = (val2 - val1 + MOD1) % MOD1;
    }
    if color[cur] == 'b' {
        dp[cur][1] = val1;
        dp[cur][2] = (val2 - val1 + MOD1) % MOD1;
    }
}

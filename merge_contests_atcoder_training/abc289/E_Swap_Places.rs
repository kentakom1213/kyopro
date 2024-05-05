//             E - Swap Places             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc289/tasks/abc289_e
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
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// `a > b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmin {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

// solve
fn main() {
    input!{T: usize}
    for _ in 0..T {
        solve();
    }
}

fn solve() {
    input!{
        N: usize,
        M: usize,
        C: [usize; N],
        edges: [(Usize1, Usize1); M],
    }

    // グラフの構築
    let graph = {
        let mut g = vec![vec![]; N];
        for &(u, v) in &edges {
            g[u].push(v);
            g[v].push(u);
        }
        g
    };

    if C[0] == C[N-1] {
        println!("-1");
        return;
    }

    // Takahashi,Aokiの動きを同時に表現するBFS
    let mut que = VecDeque::new();
    que.push_back((0, N-1));

    // dist[i][j] := Takahashiがi、Aokiがjにいるときの最短手数
    let mut dist = vec![vec![INF; N]; N];
    dist[0][N-1] = 0;

    while let Some((ct, ca)) = que.pop_front() {
        for &nt in &graph[ct] {
            for &na in &graph[ca] {
                if C[nt] != C[na] && chmin!(dist[nt][na], dist[ct][ca] + 1) {
                    que.push_back((nt, na));
                }
            }
        }
    }

    if dist[N-1][0] == INF {
        println!("-1");
    } else {
        println!("{}", dist[N-1][0]);
    }
}

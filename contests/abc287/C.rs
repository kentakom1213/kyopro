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

/// `a < b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmax {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// solve
fn main() {
    input! {
        N: usize,
        M: usize,
        UV: [(Usize1, Usize1); M],
    }

    let mut is_ok = N-1 == M;

    let mut G = vec![vec![]; N];
    for &(u, v) in &UV {
        G[u].push(v);
        G[v].push(u);
    }

    // dfsで連結判定 & dist
    let mut dist = vec![INF; N];
    dist[0] = 0;
    dfs(0, &mut dist, &G);

    // 木の直径
    let mut idx = 0;
    let mut maxi = 0;
    for (i, &d) in dist.iter().enumerate() {
        is_ok &= d < INF;
        if chmax!(maxi, d) {
            idx = i;
        }
    }

    // 2回目のdfs
    let mut dist2 = vec![INF; N];
    dist2[0] = INF;
    dfs(idx, &mut dist2, &G);

    is_ok &= *dist2.iter().max().unwrap() == N - 1;

    if is_ok {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn dfs(u: usize, dist: &mut Vec<usize>, G: &Vec<Vec<usize>>) {
    for &v in &G[u] {
        if dist[v] != INF {
            continue;
        }
        dist[v] = dist[u] + 1;
        dfs(v, dist, G);
    }
}

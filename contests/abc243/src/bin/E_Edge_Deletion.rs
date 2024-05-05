//            E - Edge Deletion
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc243/tasks/abc243_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// ## 方針
/// - ワーシャルフロイド法で全頂点対の最短経路を求める
/// - 最短経路に含まれる辺を復元する
fn main() {
    input! {
        N: usize,
        M: usize,
        edges: [(Usize1, Usize1, usize); M],
    }

    // ワーシャルフロイド法
    let mut dist = vec![vec![INF; N]; N];
    for &(u, v, c) in &edges {
        dist[u][v] = c;
        dist[v][u] = c;
    }
    for k in 0..N {
        for i in 0..N {
            for j in 0..N {
                chmin!(dist[i][j], dist[i][k] + dist[k][j],);
            }
        }
    }

    // その辺を削除した時、それ以外で最短経路を作れるか判定
    let mut ans = 0;
    for &(u, v, c) in &edges {
        let mut unused = false;
        for i in 0..N {
            unused |= dist[u][i] + dist[i][v] <= c;
        }
        if unused {
            ans += 1;
        }
    }

    println!("{}", ans);
}

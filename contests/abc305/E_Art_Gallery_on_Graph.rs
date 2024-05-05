//         E - Art Gallery on Graph
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc305/tasks/abc305_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::{VecDeque, BinaryHeap};

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
type Graph = Vec<Vec<usize>>;

// main
fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
        edges: [(Usize1, Usize1); M],
        PH: [(Usize1, usize); K],
    }

    let mut G = vec![vec![]; N];
    for &(u, v) in &edges {
        G[u].push(v);
        G[v].push(u);
    }

    // 配置されている警備員の体力
    let mut D = vec![None; N];
    for &(p, h) in &PH {
        D[p] = Some(h);
    }

    // ダイクストラ法
    let mut q: BinaryHeap<(usize, usize)> = PH.iter().map(|&(p, h)| (h, p)).collect();
    while let Some((h, p)) = q.pop() {
        for &v in &G[p] {
            if let Some(cur) = D[v] {
                if h > 0 && h - 1 > cur {
                    D[v] = Some(h - 1);
                    q.push((h - 1, v));
                }
            } else {
                if h > 0 {
                    D[v] = Some(h - 1);
                    q.push((h - 1, v));
                }
            }
        }
    }

    // 出力
    let ans = D
        .iter()
        .enumerate()
        .filter(|&(i, v)| v.is_some())
        .map(|(i, _)| i + 1)
        .collect_vec();
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}

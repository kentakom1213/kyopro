//           D - Make Bipartite 2          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc282/tasks/abc282_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// ## 方針
/// - あらかじめ、Gが2部グラフであることを確認する
/// - u,vに辺を追加した時、Gが2部グラフであるとき、以下の条件を満たす
///     - u,vの間に距離が偶数の辺が存在しない
fn main() {
    input! {
        N: usize,
        M: usize,
        edges: [(Usize1, Usize1); M],
    }

    // グラフの構築
    let mut graph = vec![vec![]; N];
    for &(a, b) in &edges {
        graph[a].push(b);
        graph[b].push(a);
    }


}
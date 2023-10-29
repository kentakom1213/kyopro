//                 G - SCC
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/practice2/tasks/practice2_g?lang=ja
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

const INF: usize = 1_000_000_000_000_000_000;
type Graph = Vec<Vec<usize>>;

// main
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        edges: [(usize, usize); M],
    }

    // グラフ、逆向きのグラフを生成
    let mut G = vec![vec![]; N];
    let mut rG = vec![vec![]; N];
    for &(u, v) in &edges {
        G[u].push(v);
        rG[v].push(u);
    }

    debug!(&G);
    debug!(&rG);

    // DFSで帰りがけ順に順序付け
    let mut order = vec![];
    let mut visited = vec![false; N];

    for i in 0..N {
        dfs(i, &G, &mut order, &mut visited);
    }

    debug!(&order);

    let mut components = vec![INF; N];
    let mut group = 0;

    // 逆向きにDFS
    for &i in order.iter().rev() {
        if components[i] == INF {
            rdfs(i, group, &rG, &mut components);
            group += 1;
        }
    }

    debug!(&components);

    // グラフの構築
    let mut compressed = vec![vec![]; group];
    for i in 0..N {
        compressed[components[i]].push(i);
    }

    debug!(&compressed);

    // 出力
    println!("{}", compressed.len());
    for to in &compressed {
        println!("{} {}", to.len(), to.iter().join(" "));
    }
}

/// 最初のDFS
fn dfs(u: usize, G: &Graph, order: &mut Vec<usize>, visited: &mut Vec<bool>) {
    if visited[u] {
        return;
    }
    visited[u] = true;
    for &v in &G[u] {
        dfs(v, G, order, visited);
    }
    order.push(u);
}

/// 2番目のDFS
fn rdfs(u: usize, group: usize, rG: &Graph, components: &mut Vec<usize>) {
    if components[u] != INF {
        return;
    }
    components[u] = group;
    for &v in &rG[u] {
        rdfs(v, group, rG, components);
    }
}

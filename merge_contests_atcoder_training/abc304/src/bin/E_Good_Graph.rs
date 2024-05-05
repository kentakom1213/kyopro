//              E - Good Graph
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc304/tasks/abc304_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::BTreeSet;

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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

type Graph = Vec<Vec<usize>>;

/// # 単純グラフ
/// - 連結成分分解
/// - 2部グラフ分解
#[derive(Debug)]
pub struct SimpleGraph {
    pub V: usize,
    pub E: usize,
    pub graph: Graph,
    pub edges: Vec<(usize, usize)>,
    pub component_size: Option<usize>,
    pub components: Vec<usize>,
}

impl SimpleGraph {
    const INF: usize = 1_000_000_000_000_000_000;

    /// グラフの構築
    pub fn new(V: usize) -> Self {
        Self {
            V,
            E: 0,
            graph: vec![vec![]; V],
            edges: vec![],
            component_size: None,
            components: vec![Self::INF; V],
        }
    }

    /// 辺の追加
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.E += 1;
        self.edges.push((u, v));
        self.graph[u].push(v);
        self.graph[v].push(u);
    }

    /// 連結成分に分解：O(|V|+|E|)
    pub fn decompose(&mut self) {
        let mut component = 0;
        self.components = vec![Self::INF; self.V];
        for i in 0..self.V {
            if self.components[i] != Self::INF {
                continue;
            }
            self.components[i] = component;
            let mut stack = vec![i];
            while let Some(u) = stack.pop() {
                for &v in &self.graph[u] {
                    if self.components[v] == Self::INF {
                        self.components[v] = component;
                        stack.push(v);
                    }
                }
            }
            component += 1;
        }
        self.component_size = Some(component);
    }

    /// 2部グラフ判定：O(|V|+|E|)
    pub fn bipartite(&mut self) -> Option<Vec<isize>> {
        // 未だ連結成分分解されていない場合
        if self.component_size.is_none() {
            self.decompose();
        }
        let mut res: Vec<isize> = vec![0; self.V];
        for i in 0..self.V {
            let mut stack = vec![i];
            if res[i] != 0 {
                continue;
            }
            res[i] = self.components[i] as isize + 1;
            while let Some(u) = stack.pop() {
                for &v in &self.graph[u] {
                    if res[v] == res[u] {
                        return None;
                    }
                    if res[v] == 0 {
                        res[v] = -res[u];
                        stack.push(v);
                    }
                }
            }
        }
        Some(res)
    }
}

// main
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        edges: [(Usize1, Usize1); M],
        K: usize,
        bad_edges: [(Usize1, Usize1); K],
        Q: usize,
        queries: [(Usize1, Usize1); Q],
    }

    let mut graph = SimpleGraph::new(N);
    for &(u, v) in &edges {
        graph.add_edge(u, v);
    }
    // 連結成分に分解
    graph.decompose();

    // 悪い辺を保存
    let mut bad = BTreeSet::new();
    for &(u, v) in &bad_edges {
        let x = graph.components[u];
        let y = graph.components[v];
        bad.insert((x, y));
        bad.insert((y, x));
    }

    // クエリの処理
    for &(u, v) in &queries {
        let x = graph.components[u];
        let y = graph.components[v];
        if bad.contains(&(x, y)) {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}

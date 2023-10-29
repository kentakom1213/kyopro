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
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:#?}, "),*), $($val),* );
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: isize = 1001001001001001001;

// main
fn main() {
    input! {
        N: usize,
        M: usize,
        edges: [(Usize1, Usize1); M],
    }

    let mut graph = SimpleGraph::new(N);
    for &(u, v) in &edges {
        graph.add_edge(u, v);
    }

    // 連結成分分解
    graph.decompose();
    let cnt = graph.component_size.unwrap();
    // 連結成分ごとの頂点数を取得
    let mut compV = vec![0; cnt];
    for &i in &graph.components {
        compV[i] += 1;
    }
    debug!(&compV);

    let mut ans = 0;
    // 連結成分を跨いで辺を張るときの組合せ数
    // (Σ 自分の頂点数 * (自分以外の頂点数)) / 2
    for &s in &compV {
        ans += s * (N - s);
    }
    ans /= 2;

    // 2部グラフ成分に分解
    if let Some(bigraph) = graph.bipartite() {
        // 連結成分ごとに、正負の要素の数をカウント
        let mut plus = vec![0; cnt];
        for &v in &bigraph {
            if v > 0 {
                plus[v as usize - 1] += 1;
            }
        }
        debug!(&plus);
        // 連結成分ごとの辺数をカウント
        let mut compE = vec![0; cnt];
        for &(u, v) in &edges {
            let c = graph.components[u];
            compE[c] += 1;
        }
        debug!(&compE);

        // 連結成分ごとに計算
        for c in 0..cnt {
            ans += plus[c] * (compV[c] - plus[c]) - compE[c];
        }

        println!("{}", ans);
    } else {
        // 2部グラフでない場合
        println!("0");
    }
}

type Graph = Vec<Vec<usize>>;

/// # 単純グラフ
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

    /// 連結成分に分解
    pub fn decompose(&mut self) {
        let mut component = 0;
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

    /// 2部グラフ判定
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

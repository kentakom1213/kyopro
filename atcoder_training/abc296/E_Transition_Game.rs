//           E - Transition Game           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc296/tasks/abc296_e
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

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

type Graph = Vec<Vec<usize>>;

/// ## SCC (強連結成分分解)
/// - Strongly Conneected Components
pub struct SCC {
    pub V: usize,
    pub E: usize,
    pub G: Graph,
    rG: Graph,
    pub group_count: Option<usize>,
    pub components: Vec<usize>,
    pub DAG: Graph,
}

impl SCC {
    const INF: usize = std::usize::MAX;

    pub fn new(V: usize) -> Self {
        Self {
            V,
            E: 0,
            G: vec![vec![]; V],
            rG: vec![vec![]; V],
            group_count: None,
            components: vec![],
            DAG: vec![],
        }
    }

    /// uからvへの有向辺を追加
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.E += 1;
        self.G[u].push(v);
        self.rG[v].push(u);
    }

    pub fn decompose(&mut self) {
        // 帰りがけ順で順序付け
        let mut order = vec![];
        let mut visited = vec![false; self.V];
        for i in 0..self.V {
            Self::dfs(i, &self.G, &mut order, &mut visited);
        }

        // 連結成分に分解
        let mut group = 0;
        let mut components = vec![Self::INF; self.V];
        for &i in order.iter().rev() {
            if components[i] == Self::INF {
                Self::rdfs(i, group, &self.rG, &mut components);
                group += 1;
            }
        }

        // DAGを構築
        let mut DAG = vec![vec![]; group];
        for i in 0..self.V {
            for &j in &self.G[i] {
                let (u, v) = (components[i], components[j]);
                if u != v {
                    DAG[u].push(v);
                }
            }
        }

        self.group_count = Some(group);
        self.components = components;
        self.DAG = DAG;
    }

    fn dfs(u: usize, G: &Graph, order: &mut Vec<usize>, visited: &mut Vec<bool>) {
        if visited[u] {
            return;
        }
        visited[u] = true;
        for &v in &G[u] {
            Self::dfs(v, G, order, visited);
        }
        order.push(u);
    }

    fn rdfs(u: usize, group: usize, rG: &Graph, components: &mut Vec<usize>) {
        if components[u] != Self::INF {
            return;
        }
        components[u] = group;
        for &v in &rG[u] {
            Self::rdfs(v, group, rG, components);
        }
    }
}

// main
fn main() {
    input! {
        N: usize,
        A: [Usize1; N],
    }

    // 強連結成分分解
    let mut scc = SCC::new(N);
    for (i, &to) in A.iter().enumerate() {
        scc.add_edge(i, to);
    }

    scc.decompose();

    debug!(&scc.components);
    debug!(&scc.G);
    debug!(&scc.DAG);

    // 各連結成分のサイズを保存
    let mut sizes = vec![0; scc.group_count.unwrap()];
    for &g in &scc.components {
        sizes[g] += 1;
    }

    debug!(&sizes);

    // 判定
    let mut ans = 0;
    for i in 0..N {
        if sizes[scc.components[i]] >= 2 || A[i] == i {
            ans += 1;
        }
    }

    println!("{}", ans);
}

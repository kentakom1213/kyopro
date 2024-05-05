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
use std::collections::{BinaryHeap, BTreeSet};
use std::{cmp::Reverse, collections::VecDeque};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

/// `chmax!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最大のものを、`x1`に代入する
/// - 代入があったとき、`true`を返す
#[macro_export]
macro_rules! chmax {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
    ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
        chmax! {
            $a,
            ($b).max($c)
            $(,$other)*
        }
    }}
}

// constant
const INF: isize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize; N],
        edges: [(Usize1, Usize1); M]
    }

    // 強連結な成分に分解
    let mut scc = SCC::new(N);

    for &(u, v) in &edges {
        if A[u] <= A[v] {
            scc.add_edge(u, v);
        }
        if A[u] >= A[v] {
            scc.add_edge(v, u);
        }
    }

    scc.decompose();

    debug!(scc);

    let NN = scc.group_count;

    // DAG上の最長路を求める
    // 辺を反転したグラフ
    let mut revG = vec![vec![]; NN];

    for u in 0..NN {
        for &v in &scc.DAG[u] {
            revG[v].push(u);
        }
    }

    debug!(revG);

    let start = scc.belongs_to[N - 1];
    let goal = scc.belongs_to[0];

    // とぽそ
    let mut indeg = vec![0; NN];

    for u in 0..NN {
        for &v in &revG[u] {
            indeg[v] += 1;
        }
    }

    let mut deq = (0..NN).filter(|&i| indeg[i] == 0).collect::<VecDeque<usize>>();

    let mut sorted = vec![];

    while let Some(u) = deq.pop_front() {
        sorted.push(u);
        for &v in &revG[u] {
            indeg[v] -= 1;
            if indeg[v] == 0 {
                deq.push_back(v);
            }
        }
    }

    debug!(sorted);

    // ソート順にDP
    let mut dist = vec![-INF; NN];
    dist[start] = 0;

    for &u in &sorted {
        for &v in &revG[u] {
            chmax! {
                dist[v],
                dist[u] + 1
            };
        }
    }

    debug!(dist);

    if dist[goal] < 0 {
        println!("0");
    } else {
        println!("{}", dist[goal] + 1);
    }
}


type Graph = Vec<Vec<usize>>;
/// ## SCC (強連結成分分解)
/// - Strongly Conneected Components
#[derive(Debug)]
pub struct SCC {
    pub V: usize,
    pub E: usize,
    pub G: Graph,
    rG: Graph,
    pub group_count: usize,
    pub belongs_to: Vec<usize>,
    pub components: Vec<Vec<usize>>,
    pub DAG: Vec<BTreeSet<usize>>,
}
impl SCC {
    const INF: usize = std::usize::MAX;
    /// 頂点`V`のグラフを構築する
    pub fn new(V: usize) -> Self {
        Self {
            V,
            E: 0,
            G: vec![vec![]; V],
            rG: vec![vec![]; V],
            group_count: 0,
            belongs_to: vec![0; V],
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
    /// 強連結成分に分解する
    pub fn decompose(&mut self) {
        // 帰りがけ順で順序付け
        let mut order = vec![];
        let mut visited = vec![false; self.V];
        for i in 0..self.V {
            Self::dfs(i, &self.G, &mut order, &mut visited);
        }
        // 連結成分に分解
        let mut group = 0;
        let mut belongs_to = vec![Self::INF; self.V];
        for &i in order.iter().rev() {
            if belongs_to[i] == Self::INF {
                Self::rdfs(i, group, &self.rG, &mut belongs_to);
                group += 1;
            }
        }
        // DAGを構築
        let mut DAG = vec![BTreeSet::new(); group];
        for i in 0..self.V {
            for &j in &self.G[i] {
                let (u, v) = (belongs_to[i], belongs_to[j]);
                if u != v {
                    DAG[u].insert(v);
                }
            }
        }
        // 分解する
        self.components.resize_with(group, Vec::new);
        for (v, &g) in belongs_to.iter().enumerate() {
            self.components[g].push(v);
        }
        self.group_count = group;
        self.belongs_to = belongs_to;
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
    fn rdfs(u: usize, group: usize, rG: &Graph, belongs_to: &mut Vec<usize>) {
        if belongs_to[u] != Self::INF {
            return;
        }
        belongs_to[u] = group;
        for &v in &rG[u] {
            Self::rdfs(v, group, rG, belongs_to);
        }
    }
}

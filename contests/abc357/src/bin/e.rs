#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::{input, marker::Usize1};

use crate::scc::SCC;

fn main() {
    input! {
        N: usize,
        A: [Usize1; N]
    }

    // 強連結成分分解
    let mut scc = SCC::new(N);

    for (i, &a) in A.iter().enumerate() {
        // i -> a
        scc.add_edge(i, a);
    }

    scc.decompose();
    debug!(scc);

    // グループ内
    let mut ans = 0_usize;

    // for g in &scc.components {
    //     ans += g.len() * (g.len() - 1) / 2 + g.len();
    // }

    // 逆辺のDAGを構成
    let NN = scc.group_count;
    let mut rDAG = vec![vec![]; NN];
    for u in 0..NN {
        for &v in &scc.DAG[u] {
            rDAG[v].push(u);
        }
    }

    // 根の集合を求める
    let mut seen = vec![false; NN];
    let mut roots = vec![];
    for mut u in 0..NN {
        while !seen[u] {
            seen[u] = true;
            if scc.DAG[u].is_empty() {
                roots.push(u);
                break;
            }
            u = scc.DAG[u][0];
        }
    }

    debug!(roots);
    debug!(rDAG);

    // 木DP的なの
    let sizes = scc.components.iter().map(|v| v.len()).collect_vec();

    debug!(sizes);

    for &u in &roots {
        let (s, res) = treedp(INF, u, &rDAG, &sizes);
        debug!(s, res);
        ans += res;
    }

    println!("{ans}");
}

fn treedp(p: usize, u: usize, G: &Vec<Vec<usize>>, sizes: &[usize]) -> (usize, usize) {
    let mut res = (0, 0);
    let s = sizes[u];

    res.0 += s;
    res.1 += s * s;

    if G[u].len() == 1 && G[u][0] == p {
        // res.0 += s;
        return res;
    }

    for &v in &G[u] {
        if v == p {
            continue;
        }
        let (sub, dp) = treedp(u, v, G, sizes);
        // 子のサイズ * 自分
        debug!((v, sub), (u, s));
        res.0 += sub;
        res.1 += dp + sub * s;
    }

    debug!(p, u, res);
    res
}

const INF: usize = 1001001001001001001;

mod macro_debug {
    #![allow(dead_code)]
    //! デバッグ用マクロ
    /// デバッグ用マクロ
    #[macro_export]
    macro_rules! debug {
        ( $($val:expr),* $(,)* ) => {{
            #[cfg(debug_assertions)]
            eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
        }};
    }
    /// 配列用マクロ
    #[macro_export]
    macro_rules! debug2D {
        ( $array:expr ) => {{
            #![cfg(debug_assertions)]
            eprintln!("{}: ", stringify!($array));
            for row in &$array {
                eprintln!("{:?}", row);
            }
        }};
    }
}

mod scc {
    #![allow(dead_code)]
    //! 強連結成分分解
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
        pub DAG: Graph,
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
            let mut DAG = vec![vec![]; group];
            for i in 0..self.V {
                for &j in &self.G[i] {
                    let (u, v) = (belongs_to[i], belongs_to[j]);
                    if u != v {
                        DAG[u].push(v);
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
}

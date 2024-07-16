//       LCA: Lowest Common Ancestor       
// ----------------------------------------
// 問題
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_C&lang=jp
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// [Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
macro_rules! get {
    ($t:ty) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($($t:ty),*) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
    ($t:ty ; $n:expr) => {
        (0..$n).map(|_|
            get!($t)
        ).collect::<Vec<_>>()
    };
    ($($t:ty),* ; $n:expr) => {
        (0..$n).map(|_|
            get!($($t),*)
        ).collect::<Vec<_>>()
    };
    ($t:ty ;;) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    ($t:ty ;; $n:expr) => {
        (0..$n).map(|_|
            get!($t ;;)
        ).collect::<Vec<_>>()
    };
}

const INF: usize = 1001001001001001001;
type Graph = Vec<Vec<usize>>;

/// # LCA
/// 最小共通祖先を求めるクエリに答える
struct LCA {
    double: Vec<Vec<usize>>,
    depth: Vec<usize>,
}

impl LCA {
    fn new(graph: &Graph, root: usize) -> Self {
        let V = graph.len();  // グラフの頂点数
        let logV = {  // log_2(グラフの頂点数)
            let mut logv = 0;
            while (V >> logv) > 0 {
                logv += 1;
            }
            logv
        };
        let mut double = vec![vec![0; V]; logV];  // ダブリング配列
        let mut depth = vec![INF; V];  // 頂点の根からの距離
        depth[0] = 0;
        Self::dfs(root, &mut double[0], &mut depth, &graph);

        // ダブリング
        for i in 1..logV {
            for j in 0..V {
                double[i][j] = double[i-1][double[i-1][j]];
            }
        }

        Self { double, depth }
    }

    fn dfs(u: usize, par: &mut Vec<usize>, depth: &mut Vec<usize>, graph: &Graph) {
        for &v in &graph[u] {
            if depth[v] != INF { continue; }
            depth[v] = depth[u] + 1;
            par[v] = u;
            Self::dfs(v, par, depth, graph);
        }
    }

    fn get_lca(&self, mut u: usize, mut v: usize) -> usize {
        // 常にuを深くする
        if self.depth[u] < self.depth[v] {
            std::mem::swap(&mut u, &mut v);
        }

        // LCAまでの距離を同じにする
        for k in 0..self.double.len() {
            if ((self.depth[u] - self.depth[v]) >> k) & 1 == 1 {
                u = self.double[k][u];
            }
        }

        if u == v {
            return u;
        }

        // 二分探索
        for k in ( 0 .. self.double.len() ).rev() {
            if self.double[k][u] != self.double[k][v] {
                u = self.double[k][u];
                v = self.double[k][v];
            }
        }
        
        self.double[0][u]
    }
}

// solve
fn main() {
    let N = get!(usize);
    let graph: Vec<Vec<usize>> = get!(usize;; N)
        .iter()
        .map(|l|
            l.iter()
             .skip(1)
             .cloned()
             .collect()
        )
        .collect();
    
    let lca = LCA::new(&graph, 0);

    // 二分探索（f(u, x) == f(v, x) となる最小のxを求める）
    let q = get!(usize);
    for _ in 0..q {
        let (u, v) = get!(usize, usize);
        let res = lca.get_lca(u, v);
        println!("{}", res);
    }
}

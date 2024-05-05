//                  D - 閉路                 
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc014/tasks/abc014_4
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
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// # LCA
/// 最小共通祖先を求めるクエリに答える
pub struct LCA {
    double: Vec<Vec<usize>>,
    depth: Vec<usize>,
}

impl LCA {
    pub fn new(graph: &Graph, root: usize) -> Self {
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

    pub fn get_lca(&self, mut u: usize, mut v: usize) -> usize {
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

    fn dist(&self, u: usize, v: usize) -> usize {
        let o = self.get_lca(u, v);
        (self.depth[u] - self.depth[o]) + (self.depth[v] - self.depth[o])
    }
}

type Graph = Vec<Vec<usize>>;

// solve
fn main() {
    input! {
        N: usize,
        xy: [(Usize1, Usize1); N-1],
        Q: usize,
        ab: [(Usize1, Usize1); Q],
    }

    let graph = {
        let mut arr = vec![vec![]; N];
        for &(x, y) in &xy {
            arr[x].push(y);
            arr[y].push(x);
        }
        arr
    };

    // LCAを求める
    let lca = LCA::new(&graph, 0);

    // クエリ処理
    for &(a, b) in &ab {
        let res = lca.dist(a, b) + 1;
        println!("{}", res);
    }
}

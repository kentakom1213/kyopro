#![allow(non_snake_case)]

use crate::cp_library_rs::{debug, graph::lca_doubling::LCA};
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(Usize1, Usize1); M],
        Q: usize,
        CD: [(Usize1, Usize1); Q],
    }

    // AB から木を構築
    let ranges = AB
        .iter()
        .enumerate()
        .flat_map(|(i, &(a, b))| [(a, i + 1), (b, i + 1)])
        .chain([(0, 0), (2 * N, 0)])
        .sorted()
        .collect_vec();

    debug!(ranges);

    let mut G = vec![vec![]; N];

    // どの頂点に属するか
    let mut vertex = vec![0; 2 * N + 2];
    let mut l = 0;
    let mut st = vec![];

    for &(r, q) in &ranges {
        if let Some(&p) = st.last() {
            if p == q {
                // スタックのトップにある値で埋める
                for j in l..=r {
                    vertex[j] = p;
                }
                l = r;
                // backtrack
                st.pop();
            } else {
                // スタックのトップにある値で埋める
                for j in l..=r {
                    vertex[j] = p;
                }
                l = r;
                st.push(q);
                // 辺を張る
                G[p].push(q);
                G[q].push(p);
            }
        } else {
            st.push(q);
        }
        debug!(st);
    }

    debug!(G);
    debug!(vertex);

    let lca = LCA::new(&G, 0);

    // クエリに答える
    for &(c, d) in &CD {
        let ans = lca.dist(vertex[c], vertex[d]);
        println!("{ans}");
    }
}

// ==================== cp-library-rs ====================
pub mod cp_library_rs {
    #![allow(dead_code)]
    pub mod graph {
        pub mod lca_doubling {
            //! ダブリングにより、最小共通祖先を求める
            const INF: usize = 1001001001001001001;
            type Graph = Vec<Vec<usize>>;
            /// # LCA
            /// 最小共通祖先を求めるクエリに答える
            pub struct LCA {
                double: Vec<Vec<usize>>,
                depth: Vec<usize>,
            }
            impl LCA {
                /// `root`を根に持つ木`tree`で、初期化を行う
                pub fn new(tree: &Graph, root: usize) -> Self {
                    let V = tree.len(); // グラフの頂点数
                    let logV = {
                        // log_2(グラフの頂点数)
                        let mut logv = 0;
                        while (V >> logv) > 0 {
                            logv += 1;
                        }
                        logv
                    };
                    let mut double = vec![vec![0; V]; logV]; // ダブリング配列
                    let mut depth = vec![INF; V]; // 頂点の根からの距離
                    depth[0] = 0;
                    Self::dfs(root, &mut double[0], &mut depth, tree);
                    // ダブリング
                    for i in 1..logV {
                        for j in 0..V {
                            double[i][j] = double[i - 1][double[i - 1][j]];
                        }
                    }
                    Self { double, depth }
                }
                fn dfs(u: usize, par: &mut Vec<usize>, depth: &mut Vec<usize>, tree: &Graph) {
                    for &v in &tree[u] {
                        if depth[v] != INF {
                            continue;
                        }
                        depth[v] = depth[u] + 1;
                        par[v] = u;
                        Self::dfs(v, par, depth, tree);
                    }
                }
                /// 頂点`u`,`v`の最小共通祖先を求める
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
                    for k in (0..self.double.len()).rev() {
                        if self.double[k][u] != self.double[k][v] {
                            u = self.double[k][u];
                            v = self.double[k][v];
                        }
                    }
                    self.double[0][u]
                }
                /// 頂点`u`,`v`の距離を求める
                pub fn dist(&self, u: usize, v: usize) -> usize {
                    let o = self.get_lca(u, v);
                    (self.depth[u] - self.depth[o]) + (self.depth[v] - self.depth[o])
                }
            }
        }
    }
    pub mod debug {
        /// 変数をデバッグ出力する
        #[macro_export]
        macro_rules! debug {
            ( $($val:expr),* $(,)* ) => {{
                #[cfg(debug_assertions)]
                eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
            }};
        }
    }
}

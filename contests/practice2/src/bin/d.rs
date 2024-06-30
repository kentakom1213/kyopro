// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::mem;

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

use crate::ford_fulkerson::FordFulkerson;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        M: usize,
        S: [Chars; N]
    }

    // A: (i+j)%2==0 であるようなマスの集合
    // B: (i+j)%2==1 であるようなマスの集合
    let mut ff = FordFulkerson::new(N * M + 2);

    let s = N * M;
    let t = s + 1;

    let idx = |r: usize, c: usize| -> usize { r * M + c };

    let mut edges = vec![];

    for i in 0..N {
        for j in 0..M {
            // 左右
            if j + 1 < M {
                let l = S[i][j];
                let r = S[i][j + 1];
                let mut a = idx(i, j);
                let mut b = idx(i, j + 1);
                if (i + j) % 2 == 1 {
                    mem::swap(&mut a, &mut b);
                }
                if l == '.' && r == '.' {
                    ff.add_edge(a, b, 1);
                    edges.push((a, b));
                }
            }
            // 上下
            if i + 1 < N {
                let t = S[i][j];
                let d = S[i + 1][j];
                let mut a = idx(i, j);
                let mut b = idx(i + 1, j);
                if (i + j) % 2 == 1 {
                    mem::swap(&mut a, &mut b);
                }
                if t == '.' && d == '.' {
                    ff.add_edge(a, b, 1);
                    edges.push((a, b));
                }
            }
        }
    }

    for i in 0..N {
        for j in 0..M {
            if (i + j) % 2 == 0 {
                ff.add_edge(s, idx(i, j), 1);
            } else {
                ff.add_edge(idx(i, j), t, 1);
            }
        }
    }

    // 最大流を求める
    let mf = ff.max_flow(s, t);

    // グリッドを求める
    let mut res = S;

    for &(mut a, mut b) in &edges {
        if ff.get_flow(a, b) == 1 {
            if a > b {
                mem::swap(&mut a, &mut b);
            }
            let (ai, aj) = (a / M, a % M);
            let (bi, bj) = (b / M, b % M);

            if aj + 1 == bj {
                res[ai][aj] = '>';
                res[bi][bj] = '<';
            } else {
                res[ai][aj] = 'v';
                res[bi][bj] = '^';
            }
        }
    }

    println!("{mf}");
    println!("{}", res.iter().map(|s| s.iter().join("")).join("\n"));
}

mod ford_fulkerson {
    use num_traits::PrimInt;
    use rustc_hash::FxHashMap;

    #[derive(Debug, Clone)]
    pub struct Edge<T> {
        /// 逆向きの辺の番号
        rev: usize,
        from: usize,
        to: usize,
        cap: T,
        original_cap: T,
    }

    impl<T: Clone> Edge<T> {
        pub fn new(rev: usize, from: usize, to: usize, cap: T) -> Self {
            Self {
                rev,
                from,
                to,
                cap: cap.clone(),
                original_cap: cap,
            }
        }
    }

    type Graph<T> = Vec<Vec<Edge<T>>>;

    /// FordFulkerson法を実行する
    #[derive(Debug)]
    pub struct FordFulkerson<T> {
        N: usize,
        /// 残余ネットワーク
        pub G: Graph<T>,
        /// 辺の番号
        edge_id: FxHashMap<(usize, usize), usize>,
    }

    impl<T: PrimInt> FordFulkerson<T> {
        pub fn new(n: usize) -> Self {
            Self {
                N: n,
                G: vec![vec![]; n],
                edge_id: FxHashMap::default(),
            }
        }

        /// 有向辺を追加する
        pub fn add_edge(&mut self, from: usize, to: usize, cap: T) {
            let rev = self.G[to].len();
            self.G[from].push(Edge::new(rev, from, to, cap));
            // 逆向き辺を追加
            let rrev = self.G[from].len() - 1;
            self.G[to].push(Edge::new(rrev, to, from, T::zero()));
            // 辺のidを追加
            self.edge_id.insert((from, to), rrev);
        }

        /// uからtへのフロー追加路を見つける
        fn dfs(u: usize, t: usize, f: T, G: &mut Graph<T>, visit: &mut [bool]) -> T {
            if u == t {
                return f;
            }
            visit[u] = true;
            for i in 0..G[u].len() {
                let v = G[u][i].to;
                let cap = G[u][i].cap;
                if visit[v] || cap <= T::zero() {
                    continue;
                }
                let d = Self::dfs(v, t, f.min(cap), G, visit);
                if d > T::zero() {
                    // 前向き辺の更新
                    G[u][i].cap = cap - d;
                    // 後ろ向き辺の更新
                    let r = G[u][i].rev;
                    G[v][r].cap = G[v][r].cap + d;

                    return d;
                }
            }
            T::zero()
        }

        /// 最大流を求める
        pub fn max_flow(&mut self, s: usize, t: usize) -> T {
            let mut flow = T::zero();
            let mut visit = vec![false; self.N];
            loop {
                visit.fill(false);
                // フロー追加路を見つける
                let f = Self::dfs(s, t, T::max_value(), &mut self.G, &mut visit);
                if f.is_zero() {
                    break;
                }
                flow = flow + f;
            }
            flow
        }

        /// 現在の状態での辺(u,v)の流量を求める
        pub fn get_flow(&self, u: usize, v: usize) -> T {
            let i = self.edge_id[&(u, v)];
            let cap = self.G[u][i].cap;
            let original = self.G[u][i].original_cap;
            original - cap
        }
    }
}

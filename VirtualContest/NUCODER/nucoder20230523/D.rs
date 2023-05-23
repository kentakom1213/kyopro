// https://atcoder.jp/contests/abc235/tasks/abc235_e

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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// [RustでUnionFind](https://zenn.dev/nakamurus/articles/f398b7f4d7618ea5b7eb)
struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
    /// 連結成分の個数
    group_count: usize,
}

impl UnionFind {
    // UnionFindを新規作成
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            siz: vec![1; n],
            group_count: n,
        }
    }

    // 根を求める
    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        self.par[x] = self.root(self.par[x]);  // 経路圧縮
        self.par[x]
    }

    // 同一の集合に所属するか判定
    fn issame(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    // 要素を結合
    fn unite(&mut self, mut parent: usize, mut child: usize) -> bool {
        parent = self.root(parent);
        child = self.root(child);

        if parent == child {
            return false;
        }

        // 要素数が大きい方を子にすることで、高さを均等に保つ
        if self.siz[parent] < self.siz[child] {
            std::mem::swap(&mut parent, &mut child);
        }

        self.par[child] = parent;
        self.siz[parent] += self.siz[child];
        self.group_count -= 1;
        true
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }
}

// main
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        Q: usize,
        edges: [(Usize1, Usize1, usize); M],
        queries: [(Usize1, Usize1, usize); Q],
    }

    let mut all_edges = edges
        .iter()
        .map(|&v| (v, INF))
        .chain(queries.iter().enumerate().map(|(i, &v)| (v, i)))
        .collect_vec();

    // 辺の重みでソート
    all_edges.sort_by_key(|(v, _)| v.2);

    // クラスカル法で最小全域木を構築
    let mut ans = vec![false; Q]; // 答えを保存
    let mut uf = UnionFind::new(N);

    for &((u, v, _), idx) in &all_edges {
        if !uf.issame(u, v) {
            if idx == INF {
                uf.unite(u, v);
            } else {
                ans[idx] = true;
            }
        }
    }


    println!("{}", ans.iter().map(|v| if *v { "Yes" } else { "No" }).join("\n"));
}

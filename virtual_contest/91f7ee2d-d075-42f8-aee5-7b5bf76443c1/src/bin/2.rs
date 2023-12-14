// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::{cmp::Reverse, collections::BinaryHeap};

// imports
use itertools::Itertools;
use proconio::{
    input,
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

fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(Usize1, Usize1); M]
    }

    let mut G = vec![vec![]; N];
    let mut indeg = vec![0; N];

    for &(a, b) in &AB {
        indeg[b] += 1;
        G[a].push(b);
    }

    // とぽそ
    let mut leaf = BinaryHeap::new();

    for i in 0..N {
        if indeg[i] == 0 {
            leaf.push(Reverse(i));
        }
    }

    // ループがないか検出
    let mut ans = vec![];

    while let Some(Reverse(u)) = leaf.pop() {
        debug!(u, leaf);
        ans.push(u + 1);
        for &v in &G[u] {
            indeg[v] -= 1;
            if indeg[v] == 0 {
                leaf.push(Reverse(v));
            }
        }
    }

    if ans.len() != N {
        println!("-1");
        return;
    }

    println!("{}", ans.iter().join(" "));
}

/// # UnionFind
pub struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
    /// 連結成分の個数
    pub group_count: usize,
}
impl UnionFind {
    /// UnionFindを新規作成
    pub fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            siz: vec![1; n],
            group_count: n,
        }
    }
    /// 根を求める
    pub fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        self.par[x] = self.root(self.par[x]); // 経路圧縮
        self.par[x]
    }
    /// 同一の集合に所属するか判定
    pub fn issame(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    /// 要素を結合
    pub fn unite(&mut self, mut parent: usize, mut child: usize) -> bool {
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
    /// 連結成分の大きさを求める
    pub fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }
}

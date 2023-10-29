// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![warn(clippy::default_numeric_fallback)]

use std::collections::BTreeSet;

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Usize1}};

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

fn main() {
    input! {
        H: usize,
        W: usize,
        S: [Chars; H],
    }

    let mut uf = UnionFind::new(H * W);

    let idx = |i, j| {
        W * i + j
    };

    for i in 0..H {
        for j in 0..W {
            if j > 0 {
                if (S[i][j], S[i][j - 1]) == ('#', '#') {
                    uf.unite(idx(i, j), idx(i, j - 1));
                }
            }
            if j < W - 1 {
                if (S[i][j], S[i][j + 1]) == ('#', '#') {
                    uf.unite(idx(i, j), idx(i, j + 1));
                }
            }
            if i > 0 {
                if (S[i][j], S[i - 1][j]) == ('#', '#') {
                    uf.unite(idx(i, j), idx(i - 1, j));
                }
            }
            if i < H - 1 {
                if (S[i][j], S[i + 1][j]) == ('#', '#') {
                    uf.unite(idx(i, j), idx(i + 1, j));
                }
            }
            if i > 0 && j > 0 {
                if (S[i][j], S[i - 1][j - 1]) == ('#', '#') {
                    uf.unite(idx(i, j), idx(i - 1, j - 1));
                }
            }
            if i > 0 && j < W - 1 {
                if (S[i][j], S[i - 1][j + 1]) == ('#', '#') {
                    uf.unite(idx(i, j), idx(i - 1, j + 1));
                }
            }
            if i < H - 1 && j > 0 {
                if (S[i][j], S[i + 1][j - 1]) == ('#', '#') {
                    uf.unite(idx(i, j), idx(i + 1, j - 1));
                }
            }
            if i < H - 1 && j < W - 1 {
                if (S[i][j], S[i + 1][j + 1]) == ('#', '#') {
                    uf.unite(idx(i, j), idx(i + 1, j + 1));
                }
            }
        }
    }

    let mut set = BTreeSet::new();

    for i in 0..H {
        for j in 0..W {
            if S[i][j] == '#' {
                set.insert(uf.root(idx(i, j)));
            }
        }
    }

    println!("{}", set.len());
}

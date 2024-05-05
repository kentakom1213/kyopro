//              D - Tying Rope             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc293/tasks/abc293_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

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
fn main() {
    input! {
        N: usize,
        M: usize,
        abcd: [(Usize1, char, Usize1, char); M],
    }

    let mut loops = BTreeSet::new();
    let mut uf = UnionFind::new(N);

    for (a, b, c, d) in abcd {
        if uf.issame(a, c) {
            loops.insert(uf.root(a));
        } else {
            uf.unite(a, c);
        }
    }

    let mut not_loop = BTreeSet::new();

    for i in 0..N {
        if !loops.contains(&uf.root(i)) {
            not_loop.insert(uf.root(i));
        }
    }

    println!("{} {}", loops.len(), not_loop.len());
}
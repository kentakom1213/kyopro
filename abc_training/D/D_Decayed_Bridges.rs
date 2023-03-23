//           D - Decayed Bridges           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc120/tasks/abc120_d
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

// [RustでUnionFind](https://zenn.dev/nakamurus/articles/f398b7f4d7618ea5b7eb)
mod uf {
    pub struct UnionFind {
        par: Vec<usize>,
        siz: Vec<usize>,
        /// 連結成分の個数
        pub group_count: usize,
    }
    
    impl UnionFind {
        // UnionFindを新規作成
        pub fn new(n: usize) -> Self {
            UnionFind {
                par: (0..n).collect(),
                siz: vec![1; n],
                group_count: n,
            }
        }
    
        // 根を求める
        pub fn root(&mut self, x: usize) -> usize {
            if self.par[x] == x {
                return x;
            }
            self.par[x] = self.root(self.par[x]);  // 経路圧縮
            self.par[x]
        }
    
        // 同一の集合に所属するか判定
        pub fn issame(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
        }
    
        // 要素を結合
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
    
        pub fn size(&mut self, x: usize) -> usize {
            let root = self.root(x);
            self.siz[root]
        }
    }
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// solve
fn main() {
    input! {
        N: usize,
        M: usize,
        edges: [(Usize1, Usize1); M],
    }

    // unionfind
    let mut uf = uf::UnionFind::new(N);

    // 答え
    let mut sum = N * (N - 1) / 2;
    let mut ans_rev = vec![sum];

    // 逆算する
    for &(u, v) in edges.iter().rev() {
        if !uf.issame(u, v) {
            let (l, r) = (uf.size(u), uf.size(v));
            sum -= l * r;
            ans_rev.push(sum);
            uf.unite(u, v);
        }
        else {
            ans_rev.push(sum);
        }
    }

    for &v in ans_rev.iter().rev().skip(1) {
        println!("{}", v);
    }
}

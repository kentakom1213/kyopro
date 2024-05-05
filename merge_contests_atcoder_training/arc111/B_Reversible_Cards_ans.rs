//           B - Reversible Cards          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc111/tasks/arc111_b
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
// https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// input macro
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

// [RustでUnionFind](https://zenn.dev/nakamurus/articles/f398b7f4d7618ea5b7eb)
struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
    /// 連結成分の個数
    cnt: usize,
}

impl UnionFind {
    // UnionFindを新規作成
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            siz: vec![1; n],
            cnt: n,
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
        self.cnt -= 1;
        true
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

const V_SIZE: usize = 404040;

/// ## 方針
/// - [解説](https://atcoder.jp/contests/arc111/editorial/519)
/// - 色を頂点としたグラフを構築し、連結成分ごとに考える
fn main() {
    let N = get!(usize);
    let edges = get!(usize, usize; N);

    // 連結成分を特定する
    let mut uf = UnionFind::new(V_SIZE);
    for &(a, b) in &edges {
        uf.unite(a, b);
    }

    // 各連結成分についてループを含むかどうかを判定する
    let mut uf2 = UnionFind::new(V_SIZE);
    let mut has_loop = vec![false; V_SIZE];
    for &(a, b) in &edges {
        if uf2.issame(a, b) {
            has_loop[uf.root(a)] = true;
        } else {
            uf2.unite(a, b);
        }
    }

    // 各連結成分を順に探索
    let mut sizes = vec![0; V_SIZE];
    for i in 0..V_SIZE {
        let v = uf.root(i);
        if sizes[v] > 0 { continue; }
        if has_loop[v] {
            sizes[v] = uf.size(v);
        } else {
            sizes[v] = uf.size(v) - 1;
        }
    }

    let ans = sizes.iter().fold(0, |acc, v| acc + v);
    println!("{}", ans);
}

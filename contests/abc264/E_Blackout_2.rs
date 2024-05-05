//              E - Blackout 2             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc264/tasks/abc264_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// imports
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
}

impl UnionFind {
    // UnionFindを新規作成
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            siz: vec![1; n],
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

/// ## 方針
/// - クエリ先読み
/// - 逆順に結合させていき、最後に出力
fn main() {
    let (N, M, E) = get!(usize, usize, usize);
    let edges = get!(usize, usize; E)
        .iter()
        .map(|&(a, b)| (min(a-1, N), min(b-1, N)))
        .collect::<Vec<(usize, usize)>>();
    let Q = get!(usize);
    let qs = get!(usize; Q)
        .iter()
        .map(|&v| v - 1)
        .collect::<Vec<usize>>();
    let q_set: HashSet<usize> = qs.iter().cloned().collect();

    // uf
    let mut uf = UnionFind::new(N+1);  // N+1個目の要素が発電所

    // クエリに含まれていない辺を結合
    for (i, &(a, b)) in edges.iter().enumerate() {
        if !q_set.contains(&i) {
            uf.unite(a, b);
        }
    }

    // 逆順に処理
    let mut ans = vec![0; Q+1];
    ans[Q] = uf.size(N) - 1;

    for i in (0..Q).rev() {
        let (a, b) = edges[qs[i]];

        uf.unite(a, b);

        ans[i] = uf.size(N) - 1;
    }

    for &v in &ans[1..] {
        println!("{}", v);
    }
}

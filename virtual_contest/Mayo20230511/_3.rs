// https://atcoder.jp/contests/abc287/tasks/abc287_c

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

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
fn main() {
    input! {
        N: usize,
        M: usize,
        edges: [(Usize1, Usize1); M],
    }

    let mut G = vec![vec![]; N];
    for &(u, v) in &edges {
        G[u].push(v);
        G[v].push(u);
    }

    // 連結性
    let is_connected = {
        let mut visited = vec![false; N];
        visited[0] = true;
        let mut st = vec![0];
        while let Some(u) = st.pop() {
            for &v in &G[u] {
                if visited[v] == false {
                    visited[v] = true;
                    st.push(v);
                }
            }
        }

        visited.iter().all(|v| *v)
    };

    debug!(is_connected);

    // ループが存在するか
    let hasnt_loop = {
        let mut has_loop = false;
        let mut uf = UnionFind::new(N);
        for &(u, v) in &edges {
            if uf.issame(u, v) {
                has_loop = true;
            }
            uf.unite(u, v);
        }
        !has_loop
    };

    debug!(hasnt_loop);

    // 次数は2以下か
    let degree_le_2 = {
        G.iter().map(|v| v.len()).max().unwrap() <= 2
    };

    debug!(degree_le_2);


    if is_connected && hasnt_loop && degree_le_2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
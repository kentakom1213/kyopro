// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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

fn main() {
    input! {
        N: usize,
        P: [Usize1; N],
        Q: [Usize1; N]
    }

    let mut uf = UnionFind::new(N);

    for (&p, &q) in P.iter().zip(Q.iter()) {
        uf.unite(p, q);
    }

    let mut cnt = vec![1; N];

    for i in 0..N {
        let root = uf.root(i);
        if cnt[root] == 1 {
            cnt[root] = uf.size(i) + (uf.size(i) > 1) as usize;
        }
    }

    let ans = cnt.iter().fold(1, |a, b| a * b % MOD);

    println!("{ans}");
}

const MOD: usize = 998_244_353;
const INF: usize = 1001001001001001001;

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

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
    marker::{Bytes, Chars, Usize1}, fastout,
};
use rustc_hash::FxHashMap;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        C: [Usize1; N],
        queries: [(usize, Usize1, Usize1); Q]
    }

    // 生徒がどの集団に属するか
    let mut uf = UnionFind::new(N);

    // group[i] := 集団iに属する生徒のクラスと、その人数
    let mut group = C
        .iter()
        .enumerate()
        .map(|(i, &c)| {
            let mut group = FxHashMap::default();
            group.insert(c, 1);
            group
        })
        .collect_vec();

    debug!(group);

    // クエリ処理
    for &(t, x, y) in &queries {
        if t == 1 {
            let x_root = uf.root(x);
            let y_root = uf.root(y);
            if uf.unite(x, y) {
                let to = uf.root(x);
                let from = x_root ^ y_root ^ to;
                // to <- fromに集合を統合
                for (k, v) in std::mem::take(&mut group[from]) {
                    *group[to].entry(k).or_insert(0) += v;
                }
            }
            debug!(group);
        } else {
            let root = uf.root(x);
            let ans = group[root].get(&y).unwrap_or(&0);
            println!("{}", ans);
        }
    }
}

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

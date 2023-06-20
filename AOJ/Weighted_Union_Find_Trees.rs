//        Weighted Union Find Trees
// ----------------------------------------
// 問題
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_B
// ----------------------------------------

use std::{fmt::Debug, io::Read};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

/// ## アーベル群
pub trait Abel {
    type E: Debug + Clone;
    const I: Self::E;
    fn op(x: &Self::E, y: &Self::E) -> Self::E;
    fn inv(x: &Self::E) -> Self::E;
}

pub struct WeightedUnionFind<G: Abel> {
    par: Vec<usize>,
    rank: Vec<usize>,
    weight: Vec<G::E>,
    pub group_count: usize,
}

impl<G: Abel> WeightedUnionFind<G> {
    /// UnionFindを構築
    pub fn new(n: usize) -> Self {
        WeightedUnionFind {
            par: (0..n).collect(),
            rank: vec![1; n],
            weight: vec![G::I; n],
            group_count: n,
        }
    }

    /// 根を求める
    pub fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        let r = self.root(self.par[x]);
        let parent = self.weight[self.par[x]].clone();
        let child = self.weight.get_mut(x).unwrap();
        *child = G::op(child, &parent);
        self.par[x] = r; // 経路圧縮
        r
    }

    /// 重みを求める
    pub fn weight(&mut self, x: usize) -> G::E {
        self.root(x); // 経路圧縮
        self.weight[x].clone()
    }

    /// 同一の集合に所属するか判定
    pub fn issame(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    /// 重みの差を求める
    /// - 同じグループにいない場合にはNone
    pub fn diff(&mut self, x: usize, y: usize) -> Option<G::E> {
        if self.issame(x, y) {
            let res = G::op(&self.weight(y), &G::inv(&self.weight(x)));
            return Some(res);
        }
        None
    }

    /// 要素を結合
    pub fn unite(&mut self, mut x: usize, mut y: usize, mut weight: G::E) -> bool {
        // x,yそれぞれについて重み差分を補正
        weight = G::op(&weight, &self.weight(x));
        weight = G::op(&weight, &G::inv(&self.weight(y)));

        x = self.root(x);
        y = self.root(y);

        if x == y {
            return false;
        }

        // 要素数が大きい方を子にすることで、高さを均等に保つ
        if self.rank[x] < self.rank[y] {
            std::mem::swap(&mut x, &mut y);
            weight = G::inv(&weight);
        }

        self.par[y] = x;
        self.rank[x] += self.rank[y];
        self.group_count -= 1;

        // 重みの更新
        self.weight[y] = weight;

        true
    }

    pub fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.rank[root]
    }
}

pub mod Alg {
    use super::Abel;

    pub struct Add;
    impl Abel for Add {
        type E = isize;
        const I: Self::E = 0;
        fn op(x: &Self::E, y: &Self::E) -> Self::E {
            x + y
        }
        fn inv(x: &Self::E) -> Self::E {
            -x
        }
    }
}

fn main() {
    // 標準入力
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s);
    let mut buf = s.split_whitespace().map(|s| s.parse::<usize>().unwrap());

    let (n, q) = (buf.next().unwrap(), buf.next().unwrap());

    // 重み付きUnionFind
    let mut uf = WeightedUnionFind::<Alg::Add>::new(n);

    // クエリ
    for _ in 0..q {
        let t = buf.next().unwrap();

        if t == 0 {
            let (x, y, z) = (
                buf.next().unwrap(),
                buf.next().unwrap(),
                buf.next().unwrap(),
            );
            uf.unite(x, y, z as isize);
        } else {
            let (x, y) = (buf.next().unwrap(), buf.next().unwrap());
            if let Some(res) = uf.diff(x, y) {
                println!("{}", res);
            } else {
                println!("?");
            }
        }

        debug!(uf.par, uf.weight);
    }
}

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

use crate::union_find::UnionFind;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

macro_rules! debug2D {
    ( $array:expr ) => {{
        #![cfg(debug_assertions)]
        eprintln!("{}: ", stringify!($array));
        for row in &$array {
            eprintln!("{:?}", row);
        }
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
    }

    let mut C = vec![0; M];
    let mut A = vec![];

    for i in 0..M {
        input! {
            k: usize,
            c: usize,
            a: [Usize1; k]
        }
        C[i] = c;
        A.push(a);
    }

    debug!(C);
    debug2D!(A);

    // Cでソートしてunionfind
    let mut uf = UnionFind::new(N);
    let mut ans = 0;

    for i in (0..M).sorted_by_key(|&i| C[i]) {
        // 同じなかま同士をつなぐ
        let u = A[i][0];
        for &v in &A[i][1..] {
            if uf.unite(u, v) {
                ans += C[i];
            }
        }
    }

    if uf.group_count() == 1 {
        println!("{ans}");
    } else {
        println!("-1");
    }
}

mod union_find {
    //! UnionFind木
    /// UnionFind木
    pub struct UnionFind {
        par: Vec<usize>,
        siz: Vec<usize>,
        /// 連結成分の個数
        count: usize,
    }
    impl UnionFind {
        /// UnionFindを新規作成
        pub fn new(n: usize) -> Self {
            UnionFind {
                par: (0..n).collect(),
                siz: vec![1; n],
                count: n,
            }
        }
        /// 根を求める
        pub fn get_root(&mut self, x: usize) -> usize {
            if self.par[x] == x {
                return x;
            }
            self.par[x] = self.get_root(self.par[x]); // 経路圧縮
            self.par[x]
        }
        /// 同一の集合に所属するか判定
        pub fn is_same(&mut self, x: usize, y: usize) -> bool {
            self.get_root(x) == self.get_root(y)
        }
        /// 要素を結合
        pub fn unite(&mut self, mut parent: usize, mut child: usize) -> bool {
            parent = self.get_root(parent);
            child = self.get_root(child);
            if parent == child {
                return false;
            }
            // 要素数が大きい方を子にすることで、高さを均等に保つ
            if self.siz[parent] < self.siz[child] {
                std::mem::swap(&mut parent, &mut child);
            }
            self.par[child] = parent;
            self.siz[parent] += self.siz[child];
            self.count -= 1;
            true
        }
        /// 連結成分の大きさを求める
        pub fn get_size(&mut self, x: usize) -> usize {
            let get_root = self.get_root(x);
            self.siz[get_root]
        }
        /// 連結成分の数を返す
        pub fn group_count(&self) -> usize {
            self.count
        }
    }
}

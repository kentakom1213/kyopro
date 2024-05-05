#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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

use std::{
    collections::{BTreeMap, HashMap},
    mem,
};

use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

use crate::union_find::UnionFind;

fn main() {
    input! {
        N: usize,
        Q: usize,
    }

    // ボールiについてのUnionFind
    let mut uf = UnionFind::new(N + Q + 3);

    // 箱xに入っている集合の代表元（空のときNone）
    let mut ld = (0..=N).map(|v| Some(v)).collect_vec();

    // 代表元がyである集合がどの箱に入っているか
    let mut inv = (0..N + Q + 3).collect_vec();

    // 現在のボールの個数
    let mut cnt = N;

    for _ in 0..Q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    x: usize,
                    y: usize,
                }
                match (ld[x], ld[y]) {
                    (Some(lx), Some(ly)) => {
                        // 併合
                        uf.unite(lx, ly);
                        // 新しい根
                        let root = uf.get_root(lx);
                        ld[x] = Some(root);
                        // 箱の更新
                        inv[root] = x;
                        // 箱yを空に
                        ld[y] = None;
                    }
                    (None, Some(ly)) => {
                        // 中身を移す
                        ld[x] = Some(ly);
                        // 箱の更新
                        inv[ly] = x;
                        // 箱yを空に
                        ld[y] = None;
                    }
                    (_, None) => {
                        // 何もしなくて良い
                    }
                }
            }
            2 => {
                input! {
                    x: usize,
                }
                cnt += 1;
                // 箱に入っているとき
                if let Some(lx) = ld[x] {
                    // マージ
                    uf.unite(lx, cnt);
                    // 新しい代表元
                    let root = uf.get_root(lx);
                    ld[x] = Some(root);
                    // 箱を更新
                    inv[root] = x;
                }
                // 空のとき
                else {
                    let root = uf.get_root(cnt);
                    // 箱に入れる
                    ld[x] = Some(root);
                    // 箱を更新
                    inv[root] = x;
                }
            }
            3 => {
                input! {
                    x: usize,
                }
                // 代表元を取得
                let lx = uf.get_root(x);
                // 箱を取得
                let bx = inv[lx];

                println!("{bx}");
            }
            _ => (),
        }
        debug!(uf.par);
        debug!(ld);
        debug!(inv);
    }
}

const INF: usize = 1001001001001001001;

mod union_find {
    //! UnionFind木
    /// UnionFind木
    pub struct UnionFind {
        pub par: Vec<usize>,
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

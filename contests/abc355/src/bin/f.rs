#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

use crate::union_find::UnionFind;

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        ABC: [(Usize1, Usize1, usize); N - 1],
        UVW: [(Usize1, Usize1, usize); Q]
    }

    // ufs[i] := 重みi以上の辺が張られている連結成分のクラス
    let mut ufs = (0..=10).map(|_| UnionFind::new(N)).collect_vec();

    // 構築
    for &(u, v, w) in &ABC {
        for i in w..=10 {
            ufs[i].unite(u, v);
        }
    }

    // クエリ処理
    for &(u, v, w) in &UVW {
        // 辺の追加
        for i in w..=10 {
            ufs[i].unite(u, v);
        }

        // 連結成分の個数をカウント
        let ans = (0..=10).map(|i| ufs[i].group_count() - 1).sum::<usize>();

        println!("{ans}");
    }
}

const _INF: usize = 1001001001001001001;

mod union_find {
    #![allow(dead_code)]
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

mod macro_debug {
    #![allow(dead_code)]
    //! デバッグ用マクロ
    /// デバッグ用マクロ
    #[macro_export]
    macro_rules! debug {
        ( $($val:expr),* $(,)* ) => {{
            #[cfg(debug_assertions)]
            eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
        }};
    }
    /// 配列用マクロ
    #[macro_export]
    macro_rules! debug2D {
        ( $array:expr ) => {{
            #![cfg(debug_assertions)]
            eprintln!("{}: ", stringify!($array));
            for row in &$array {
                eprintln!("{:?}", row);
            }
        }};
    }
}

//              B - Picking Up             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/diverta2019-2/tasks/diverta2019_2_b
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

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

use std::ops::{Add, Sub, Mul, Neg};

type Pos<T> = (T, T);
type Line<T> = (Pos<T>, Pos<T>);
type P = Pos<isize>;

trait Vec2<T> {
    fn mul(&self, scalar: T) -> Self;
    fn add(&self, other: Self) -> Self;
    fn sub(&self, other: Self) -> Self;
    fn dot(&self, other: Self) -> T;
    fn cross(&self, other: Self) -> T;
    fn dist2(&self, other: Self) -> T;
}

impl<T> Vec2<T> for Pos<T>
where T: Copy
    + Add<Output = T>
    + Sub<Output = T>
    + Mul<Output = T>
    + Neg
{
    fn mul(&self, scalar: T) -> Self {
        (self.0 * scalar, self.1 * scalar)
    }
    fn add(&self, other: Self) -> Self {
        (self.0 + other.0, self.1 + other.1)
    }
    fn sub(&self, other: Self) -> Self {
        (self.0 - other.0, self.1 - other.1)
    }
    fn dot(&self, other: Self) -> T {
        self.0 * other.0 + self.1 * other.1
    }
    /// ## cross
    /// ベクトルのクロス積
    fn cross(&self, other: Self) -> T {
        (self.0 * other.1) - (other.0 * self.1)
    }
    /// ## dist2
    /// 距離の2乗の値を返す
    fn dist2(&self, other: Self) -> T {
        (self.0 - other.0) * (self.0 - other.0)
        + (self.1 - other.1) * (self.1 - other.1)
    }
}

/// ## 方針
/// - 傾きによって分類し、組の数が最も小さくなるようにする
fn main() {
    input! {
        N: usize,
        XY: [(isize, isize); N],
    }

    if N == 1 {
        println!("1");
        return;
    }

    let mut ans = INF;

    for i in 0..N {
        for j in i+1..N {
            let t = XY[i].sub(XY[j]);
            
            // 傾きで分類
            let mut uf = UnionFind::new(N);

            for k in 0..N {
                for l in 0..N {
                    if XY[k].add(t) == XY[l] {
                        uf.unite(k, l);
                    }
                }
            }

            if uf.cnt < ans {
                ans = uf.cnt;
            }
        }
    }

    println!("{}", ans);
}

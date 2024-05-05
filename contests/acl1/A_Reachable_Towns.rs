//           A - Reachable Towns           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/acl1/tasks/acl1_a
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
    ($($t:ty);*) => {
        (
            $(get!($t),)*
        )
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

type Pint = (usize, usize);

/// ## 解説AC
/// - [けんちょんさんの解説](https://drken1215.hatenablog.com/entry/2020/09/23/080100)
fn main() {
    let N = get!(usize);
    let xy: Vec<Pint> = get!(usize, usize; N)
        .iter()
        .map(|(x, y)| (x-1, y-1))
        .collect();
    let mut Y: Vec<usize> = vec![0; N];
    let mut X: Vec<usize> = vec![0; N];

    // xの値でソート `O(N)`
    for (i, &(x, y)) in xy.iter().enumerate() {
        Y[x] = y;
        X[i] = x;
    }

    let mut uf = UnionFind::new(N);

    let mut st: VecDeque<Pint> = VecDeque::new();
    let mut ymin = (Y[0], 0);
    for (x, &y) in Y.iter().enumerate() {
        if ymin > (y, x) {
            st.push_back(ymin);
            ymin = (y, x);
        } else {
            uf.unite(x, ymin.1);
            while let Some(&(y2, x2)) = st.back() {
                if y2 < y {
                    uf.unite(x, x2);
                    st.pop_back();
                } else {
                    break;
                }
            }
        }
    }

    for &x in &X {
        println!("{}", uf.size(x));
    }
}

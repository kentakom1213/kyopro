//        Range Minimum Query (RMQ)        
// ----------------------------------------
// 問題
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A
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

use std::ops::{
    Bound::{Excluded, Included, Unbounded},
    Index, RangeBounds,
};

/// SegmentTree (Monoid)
struct SegmentTree<F, T> {
    len: usize,
    offset: usize,
    data: Vec<T>,
    op: F,
    e: T,
}

impl<F, T> Index<usize> for SegmentTree<F, T> {
    type Output = T;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[self.offset + idx]
    }
}

impl<F, T> SegmentTree<F, T>
where
    F: Fn(T, T) -> T,
    T: Copy + Eq + std::fmt::Debug,
{
    /// ## new
    /// セグメント木を初期化する
    fn new(n: usize, e: T, op: F) -> Self {
        let len = n.next_power_of_two();

        Self {
            len: len,
            offset: len,
            data: vec![e; len << 1],
            op: op,
            e: e,
        }
    }

    /// ## from
    /// 配列からセグメント木を生成する
    fn from(arr: &[T], e: T, op: F) -> Self {
        let mut seg = Self::new(arr.len(), e, op);
        for (i, &v) in arr.iter().enumerate() {
            seg.data[seg.offset + i] = v;
        }
        for i in (0..seg.offset).rev() {
            let lch = i << 1;
            seg.data[i] = (seg.op)(seg.data[lch], seg.data[lch + 1]);
        }
        seg
    }

    /// ## update
    /// 要素`index`を`value`に上書きする
    /// （`index`：0-indexed）
    fn update(&mut self, index: usize, value: T) {
        let mut i = index + self.offset;
        self.data[i] = value;
        while i > 1 {
            i >>= 1;
            let lch = i << 1;
            self.data[i] = (self.op)(self.data[lch], self.data[lch + 1]);
        }
    }

    fn get_range<R: RangeBounds<usize>>(&self, range: R) -> T {
        let start = match range.start_bound() {
            Unbounded => 0,
            Excluded(&v) => v + 1,
            Included(&v) => v,
        };
        let end = match range.end_bound() {
            Unbounded => self.len,
            Excluded(&v) => v,
            Included(&v) => v - 1,
        };
        assert!(start <= end);
        // 全体の値を取得
        if (start, end) == (0, self.len) {
            return self.data[1];
        }

        // 値の取得
        let mut l = self.offset + start;
        let mut r = self.offset + end;
        let (mut res_l, mut res_r) = (self.e, self.e);

        while l < r {
            if l & 1 == 1 {
                res_l = (self.op)(res_l, self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                res_r = (self.op)(self.data[r], res_r);
            }
            l >>= 1;
            r >>= 1;
        }

        (self.op)(res_l, res_r)
    }
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// solve
fn main() {
    let (N, Q) = get!(usize, usize);

    const INF: usize = (1 << 31) - 1;
    let mut segtree = SegmentTree::new(N, INF, |a, b| a.min(b));

    for _ in 0..Q {
        let (com, x, y) = get!(usize, usize, usize);
        if com == 0 {
            segtree.update(x, y);
        } else {
            let ans = segtree.get_range(x .. y+1);
            println!("{}", ans);
        }
    }
}

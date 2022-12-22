// Product of Ranges
// ------------------
// 問題
// https://mojacoder.app/users/powell/problems/product_of_ranges
// ------------------

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

const MOD: usize = 998_244_353;
// const MOD: usize = 1_000_000_007;

trait Modint {
    fn val(&self) -> usize;
    fn madd(&self, other: usize) -> usize;
    fn mneg(&self) -> usize;
    fn msub(&self, other: usize) -> usize;
    fn mmul(&self, other: usize) -> usize;
    fn minv(&self) -> usize;
    fn mdiv(&self, other: usize) -> usize;
    fn mpow(&self, other: usize) -> usize;
}

impl Modint for usize {
    fn val(&self) -> usize {
        self % MOD
    }

    fn madd(&self, other: usize) -> usize {
        (self.val() + other.val()).val()
    }

    fn mneg(&self) -> usize {
        (MOD - self.val()).val()
    }

    fn msub(&self, other: usize) -> usize {
        self.madd(other.mneg())
    }

    fn mmul(&self, other: usize) -> usize {
        (self.val() * other.val()).val()
    }

    fn mpow(&self, other: usize) -> usize {
        let (mut a, mut b) = (self.val(), other);
        let mut res = 1;
        while b > 0 {
            if b & 1 == 1 {
                res = res.mmul(a);
            }
            a = a.mmul(a);
            b >>= 1;
        }
        res
    }

    fn minv(&self) -> usize{
        assert!(*self != 0);
        self.mpow(MOD - 2)
    }

    fn mdiv(&self, other: usize) -> usize {
        self.mmul(other.minv())
    }
}

/// SegmentTree (Monoid)
struct SegmentTree<F, T> {
    offset: usize,
    data: Vec<T>,
    op: F,
    e: T,
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

    /// ## get_point
    /// 一点取得する
    fn get_point(&self, index: usize) -> T {
        let i = self.offset + index;
        self.data[i]
    }

    /// ## get_range
    /// 区間`[l, r)`を取得する
    fn get_range(&self, left: usize, right: usize) -> T {
        let mut l = self.offset + left;
        let mut r = self.offset + right;
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
    let A = get!(usize;;);
    let queries = get!(usize, usize; Q);

    let seg = SegmentTree::from(
        &A,
        1_usize,
        |a, b| a.mmul(b),
    );

    for &(a, b) in &queries {
        let res = seg.get_range(a-1, b);
        println!("{}", res);
    }
}

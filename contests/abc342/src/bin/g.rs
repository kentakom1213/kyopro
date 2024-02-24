#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use im_rc::{vector, Vector};
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

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
        A: [isize; N],
        Q: usize,
    }

    // 永続セグ木
    let mut seg = {
        let mut seg = Vector::new();

        for _ in 0..OFFSET {
            seg.push_back(isize::MIN);
        }

        for &a in &A {
            seg.push_back(a);
        }

        seg
    };

    let mut logs = HashMap::new();

    for i in 0..Q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    l: Usize1,
                    r: usize,
                    x: isize
                }
                apply_range(&mut seg, l, r, x);
                logs.insert(i, seg.clone());
                debug!(logs);
            }
            2 => {
                input! {
                    i: Usize1,
                }
            }
            3 => {
                input! {
                    i: Usize1,
                }
                let ans = logs[&i][i];
                println!("{ans}");
            }
            _ => (),
        }
    }
}

use std::{
    collections::HashMap,
    ops::{
        Bound::{Excluded, Included, Unbounded},
        RangeBounds,
    },
};

const OFFSET: usize = 20;

fn op(a: &isize, b: &isize) -> isize {
    *a.min(b)
}

/// 一点取得
pub fn get_point(data: &mut Vector<isize>, index: usize) -> isize {
    let mut i = OFFSET + index;
    let mut res = data[i].clone();
    while i > 1 {
        i >>= 1;
        let tmp = op(&data[i], &res);
        res = tmp;
    }
    res
}

/// 区間更新:
/// - 区間`range`を`x`で更新する
pub fn apply_range(data: &mut Vector<isize>, start: usize, end: usize, x: isize) {
    // 値の更新
    let mut l = OFFSET + start;
    let mut r = OFFSET + end;
    while l < r {
        if l & 1 == 1 {
            let tmp = op(&data[l], &x);
            data[l] = tmp;
            l += 1;
        }
        if r & 1 == 1 {
            r -= 1;
            let tmp = op(&data[r], &x);
            data[r] = tmp;
        }
        l >>= 1;
        r >>= 1;
    }
}

mod dual_segment_tree {
    //! 双対セグメント木：区間加算・一点取得
    use std::fmt::{self, Debug};
    use std::ops::{
        Bound::{Excluded, Included, Unbounded},
        RangeBounds,
    };
    /// 可換モノイド
    pub trait CommutativeMonoid {
        /// 元の型
        type Val: fmt::Debug + Clone + PartialEq;
        /// 単位元
        const E: Self::Val;
        /// 演算
        fn op(arg1: &Self::Val, arg2: &Self::Val) -> Self::Val;
    }
    /// 双対セグ木
    /// - 区間への作用
    /// - 一点の取得
    /// を行うセグメント木
    pub struct DualSegmentTree<M: CommutativeMonoid> {
        pub size: usize,
        offset: usize,
        data: Vec<M::Val>,
    }
    impl<M: CommutativeMonoid> DualSegmentTree<M> {
        #[inline]
        fn parse_range<R: RangeBounds<usize>>(&self, range: &R) -> Option<(usize, usize)> {
            let start = match range.start_bound() {
                Unbounded => 0,
                Excluded(&v) => v + 1,
                Included(&v) => v,
            };
            let end = match range.end_bound() {
                Unbounded => self.size,
                Excluded(&v) => v,
                Included(&v) => v + 1,
            };
            if start <= end && end <= self.size {
                Some((start, end))
            } else {
                None
            }
        }
        /// 双対セグメント木を初期化する
        pub fn new(n: usize) -> Self {
            let offset = n;
            Self {
                size: n,
                offset,
                data: vec![M::E; offset << 1],
            }
        }
        /// 配列から双対セグメント木を構築する
        pub fn build(arr: &Vec<M::Val>) -> Self {
            let offset = arr.len();
            let mut seg = Self::new(offset);
            seg.data[offset..].clone_from_slice(arr);
            seg
        }
        /// 区間更新:
        /// - 区間`range`を`x`で更新する
        pub fn apply_range<R: RangeBounds<usize> + Debug>(&mut self, range: R, x: M::Val) {
            let Some((start, end)) = self.parse_range(&range) else {
                panic!("The given range is wrong: {:?}", range);
            };
            // 値の更新
            let mut l = self.offset + start;
            let mut r = self.offset + end;
            while l < r {
                if l & 1 == 1 {
                    let tmp = M::op(&self.data[l], &x);
                    self.data[l] = tmp;
                    l += 1;
                }
                if r & 1 == 1 {
                    r -= 1;
                    let tmp = M::op(&self.data[r], &x);
                    self.data[r] = tmp;
                }
                l >>= 1;
                r >>= 1;
            }
        }
        /// 一点取得
        pub fn get_point(&self, index: usize) -> M::Val {
            let mut i = index + self.offset;
            let mut res = self.data[i].clone();
            while i > 1 {
                i >>= 1;
                let tmp = M::op(&self.data[i], &res);
                res = tmp;
            }
            res
        }
    }
    impl<M: CommutativeMonoid + Debug> Debug for DualSegmentTree<M> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "DualSegmentTree {{ [").ok();
            for i in 0..self.size {
                if i + 1 < self.size {
                    write!(f, "{:?}, ", self.get_point(i)).ok();
                } else {
                    write!(f, "{:?}", self.get_point(i)).ok();
                }
            }
            write!(f, "] }}")
        }
    }
    pub mod Alg {
        use super::CommutativeMonoid;
        use std::fmt::Debug;
        /// 整数の和
        #[derive(Debug)]
        pub struct Add;
        impl CommutativeMonoid for Add {
            type Val = isize;
            const E: Self::Val = 0;
            fn op(arg1: &Self::Val, arg2: &Self::Val) -> Self::Val {
                arg1 + arg2
            }
        }
        /// bit単位の排他的論理和
        pub struct Xor;
        impl CommutativeMonoid for Xor {
            type Val = usize;
            const E: Self::Val = 0;
            fn op(arg1: &Self::Val, arg2: &Self::Val) -> Self::Val {
                arg1 ^ arg2
            }
        }
        /// chmax操作
        #[derive(Debug)]
        pub struct ChMax;
        impl CommutativeMonoid for ChMax {
            type Val = isize;
            const E: Self::Val = isize::MIN;
            fn op(arg1: &Self::Val, arg2: &Self::Val) -> Self::Val {
                *arg1.max(arg2)
            }
        }
    }
}

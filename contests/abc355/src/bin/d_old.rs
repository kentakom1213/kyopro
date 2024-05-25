#![allow(non_snake_case)]

use proconio::{input, marker::Isize1};
use superslice::Ext;

use crate::{extmonoid::examples::RMQandRAQ, lazy_segment_tree::LazySegmentTree};

fn main() {
    input! {
        N: usize,
        mut LR: [(Isize1, isize); N]
    }

    // 区間をソート
    LR.sort_by_key(|&(l, r)| l);

    // 座標圧縮
    let comp = {
        let mut arr = vec![];
        for &(l, r) in &LR {
            arr.push(l);
            arr.push(r);
        }
        arr.sort();
        arr.dedup();
        arr
    };

    debug!(comp);

    // 区間加算，区間最大
    let mut seg = LazySegmentTree::<RMQandRAQ>::from(&vec![0; comp.len() + 1]);

    debug!(seg);

    let mut cnt = 0;

    for &(l, r) in &LR {
        let l = comp.lower_bound(&l);
        let r = comp.lower_bound(&r);

        // 区間最大
        let mx = seg.get(l..r);
        debug!(mx);
        cnt += mx;

        // 区間加算
        seg.apply(l..r, 1);
    }

    println!("{cnt}");
}

mod extmonoid {
    #![allow(dead_code)]
    //! 作用付きモノイド
    /// 作用付きモノイド
    pub trait ExtMonoid {
        /// 要素のデータ型
        type X: Clone + PartialEq;
        /// 作用素のデータ型
        type M: Clone + PartialEq;
        /// 要素Xの単位元
        const IX: Self::X;
        /// 作用素Mの単位元
        const IM: Self::M;
        /// 要素同士の演算
        fn operate_x(x: &Self::X, y: &Self::X) -> Self::X;
        /// 要素に対する作用
        fn apply(x: &Self::X, y: &Self::M) -> Self::X;
        /// 作用素同士の演算
        fn operate_m(x: &Self::M, y: &Self::M) -> Self::M;
        /// 作用素の集約
        fn aggregate(x: &Self::M, p: usize) -> Self::M;
    }
    /// （遅延セグ木）作用付きモノイド
    pub mod examples {
        use super::ExtMonoid;
        /// ## RSQandRAQ
        /// - 区間加算
        /// - 区間和
        #[derive(Debug)]
        pub struct RSQandRAQ;
        impl ExtMonoid for RSQandRAQ {
            type X = isize;
            type M = isize;
            const IX: Self::X = 0;
            const IM: Self::M = 0;
            fn operate_x(x: &Self::X, y: &Self::X) -> Self::X {
                x + y
            }
            fn apply(x: &Self::X, y: &Self::M) -> Self::X {
                x + y
            }
            fn operate_m(x: &Self::M, y: &Self::M) -> Self::M {
                x + y
            }
            fn aggregate(x: &Self::M, p: usize) -> Self::M {
                x * p as isize
            }
        }
        /// ## RMQandRUQ
        /// - 区間更新
        /// - 区間最小値
        #[derive(Debug)]
        pub struct RMQandRUQ;
        impl ExtMonoid for RMQandRUQ {
            type X = isize;
            type M = isize;
            const IM: Self::M = (1 << 31) - 1;
            const IX: Self::X = (1 << 31) - 1;
            fn operate_x(x: &Self::X, y: &Self::X) -> Self::X {
                *x.min(y)
            }
            fn apply(_x: &Self::X, y: &Self::M) -> Self::X {
                *y
            }
            fn operate_m(_x: &Self::M, y: &Self::M) -> Self::M {
                *y
            }
            fn aggregate(x: &Self::M, _p: usize) -> Self::M {
                *x
            }
        }
        /// ## RMQandRAQ
        /// - 区間加算
        /// - 区間最小値
        #[derive(Debug)]
        pub struct RMQandRAQ;
        impl ExtMonoid for RMQandRAQ {
            type X = isize;
            type M = isize;
            const IM: Self::M = 0;
            const IX: Self::X = -(1 << 31);
            fn operate_x(x: &Self::X, y: &Self::X) -> Self::X {
                *x.max(y)
            }
            fn apply(x: &Self::X, y: &Self::M) -> Self::X {
                x + y
            }
            fn operate_m(x: &Self::M, y: &Self::M) -> Self::M {
                x + y
            }
            fn aggregate(x: &Self::M, _p: usize) -> Self::M {
                *x
            }
        }
        /// ## RSQandRUQ
        /// - 区間更新
        /// - 区間和取得
        #[derive(Debug)]
        pub struct RSQandRUQ;
        impl ExtMonoid for RSQandRUQ {
            type X = isize;
            type M = Option<isize>;
            const IX: Self::X = 0;
            const IM: Self::M = None;
            fn operate_x(x: &Self::X, y: &Self::X) -> Self::X {
                x + y
            }
            fn apply(_x: &Self::X, y: &Self::M) -> Self::X {
                y.unwrap()
            }
            fn operate_m(_x: &Self::M, y: &Self::M) -> Self::M {
                *y
            }
            fn aggregate(x: &Self::M, p: usize) -> Self::M {
                x.map(|x| x * p as isize)
            }
        }
    }
}

mod lazy_segment_tree {
    #![allow(dead_code)]
    //! 遅延評価セグメント木
    use crate::extmonoid::ExtMonoid;
    use core::fmt;
    use std::{
        fmt::Debug,
        ops::{
            Bound::{Excluded, Included, Unbounded},
            RangeBounds,
        },
    };
    /// 遅延評価セグメント木
    #[derive(Debug)]
    pub struct LazySegmentTree<M: ExtMonoid> {
        pub size: usize,
        offset: usize,
        data: Vec<M::X>,
        lazy: Vec<M::M>,
    }
    impl<M: ExtMonoid> LazySegmentTree<M> {
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
        /// 遅延評価セグメント木を初期化する
        /// - `n`: 配列サイズ
        pub fn new(n: usize) -> Self {
            let offset = n.next_power_of_two();
            Self {
                size: n,
                offset,
                data: vec![M::IX; offset << 1],
                lazy: vec![M::IM; offset << 1],
            }
        }
        /// 遅延値を評価
        fn eval(&mut self, idx: usize, len: usize) {
            if self.lazy[idx] == M::IM {
                return;
            }
            // 葉でなければ子に伝搬
            if idx < self.offset {
                self.lazy[idx * 2] = M::operate_m(&self.lazy[idx * 2], &self.lazy[idx]);
                self.lazy[idx * 2 + 1] = M::operate_m(&self.lazy[idx * 2 + 1], &self.lazy[idx]);
            }
            // 自身を更新
            self.data[idx] = M::apply(&self.data[idx], &M::aggregate(&self.lazy[idx], len));
            self.lazy[idx] = M::IM;
        }
        /// 区間に`val`を作用させる
        /// - `range`: `[left, right)`
        pub fn apply<R: RangeBounds<usize> + fmt::Debug>(&mut self, range: R, val: M::M) {
            let Some((left, right)) = self.parse_range(&range) else {
                panic!("The given range is wrong: {:?}", range);
            };
            self.apply_inner(left, right, val, 0, self.offset, 1);
        }
        fn apply_inner(
            &mut self,
            left: usize,
            right: usize,
            val: M::M,
            begin: usize,
            end: usize,
            idx: usize,
        ) {
            // 遅延値を評価
            self.eval(idx, end - begin);
            // 区間を内包するとき
            if left <= begin && end <= right {
                self.lazy[idx] = M::operate_m(&self.lazy[idx], &val);
                self.eval(idx, end - begin);
            }
            // 区間が重なるとき
            else if left < end && begin < right {
                let mid = (begin + end) / 2;
                // 左の子を更新
                self.apply_inner(left, right, val.clone(), begin, mid, idx * 2);
                // 右の子を更新
                self.apply_inner(left, right, val, mid, end, idx * 2 + 1);
                // 値を更新
                self.data[idx] = M::operate_x(&self.data[idx * 2], &self.data[idx * 2 + 1]);
            }
        }
        /// 区間を取得する
        /// - `range`: `[left, right)`
        pub fn get<R: RangeBounds<usize> + fmt::Debug>(&mut self, range: R) -> M::X {
            let Some((left, right)) = self.parse_range(&range) else {
                panic!("The given range is wrong: {:?}", range);
            };
            self.get_inner(left, right, 0, self.offset, 1)
        }
        fn get_inner(
            &mut self,
            left: usize,
            right: usize,
            begin: usize,
            end: usize,
            idx: usize,
        ) -> M::X {
            // 遅延値を評価
            self.eval(idx, end - begin);
            // 区間を含まない
            if end <= left || right <= begin {
                M::IX
            }
            // 区間を包含する
            else if left <= begin && end <= right {
                self.data[idx].clone()
            }
            // 区間が重なる
            else {
                let mid = (begin + end) / 2;
                let l_val = self.get_inner(left, right, begin, mid, idx * 2);
                let r_val = self.get_inner(left, right, mid, end, idx * 2 + 1);
                M::operate_x(&l_val, &r_val)
            }
        }
    }
    impl<M: ExtMonoid> From<&Vec<M::X>> for LazySegmentTree<M> {
        fn from(src: &Vec<M::X>) -> Self {
            let mut seg = Self::new(src.len());
            for (i, v) in src.iter().enumerate() {
                seg.data[seg.offset + i] = v.clone();
            }
            for i in (0..seg.offset).rev() {
                let lch = i << 1;
                seg.data[i] = M::operate_x(&seg.data[lch], &seg.data[lch + 1]);
            }
            seg
        }
    }
    impl<M> LazySegmentTree<M>
    where
        M: ExtMonoid,
        M::M: Debug,
        M::X: Debug,
    {
        pub fn show(&mut self) -> String {
            let mut res = "LazySegmentTree {{ [".to_string();
            for i in 0..self.size {
                if i + 1 < self.size {
                    res += &format!("{:?}, ", self.get(i..=i));
                } else {
                    res += &format!("{:?}", self.get(i..=i));
                }
            }
            res += "] }}";
            res
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

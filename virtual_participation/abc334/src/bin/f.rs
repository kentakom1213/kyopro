#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use itertools::Itertools;
use monoid::Monoid;
use num_traits::Pow;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

use crate::segment_tree::SegmentTree;

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
        K: usize,
        S: (isize, isize),
        mut HOMES: [(isize, isize); N]
    }

    // サンタさんの家を追加
    HOMES.push(S);

    // 最終的な最短路
    let mut ans = dist(S, HOMES[0]);

    // 差分のリスト
    let mut diffs = vec![];

    for i in 0..N {
        let cur = HOMES[i];
        let nxt = HOMES[i + 1];

        // 直接次の家に向かう
        let to = dist(cur, nxt);
        // 一旦自宅に帰る
        let via = dist(cur, S) + dist(S, nxt);

        ans += to;
        diffs.push(via - to);
    }

    debug!(ans);
    debug!(diffs);

    // 間隔がKを超えないように、差分のリストからいくつか選ぶときの和の最小値
    let mut dp = SegmentTree::<Min>::new(N + 1);
    *dp.get_mut(0).unwrap() = 0.0;

    for i in 1..=N {
        *dp.get_mut(i).unwrap() = dp.get_range(i.saturating_sub(K)..i) + diffs[i - 1];
    }

    debug!(dp);

    // 差分の和の最小値を足す
    ans += dp[N];

    println!("{ans:.15}");
}

struct Min;
impl Monoid for Min {
    type Val = f64;
    const E: Self::Val = 1e20;
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        left.min(*right)
    }
}

/// 2点間の距離
fn dist((sx, sy): (isize, isize), (tx, ty): (isize, isize)) -> f64 {
    let d2 = (sx - tx).pow(2) + (sy - ty).pow(2);
    (d2 as f64).sqrt()
}

mod monoid {
    //! モノイド
    use std::fmt::Debug;
    /// モノイド
    pub trait Monoid {
        /// 元の型
        type Val: Debug + Clone + PartialEq;
        /// 単位元
        const E: Self::Val;
        /// 演算
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val;
    }
    /// 各種モノイド
    pub mod examples {
        use super::Monoid;
        /// 和
        pub struct Add;
        impl Monoid for Add {
            type Val = isize;
            const E: Self::Val = 0;
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                left + right
            }
        }
        /// 積
        pub struct Mul;
        impl Monoid for Mul {
            type Val = isize;
            const E: Self::Val = 1;
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                left * right
            }
        }
        /// bit単位の排他的論理和
        pub struct Xor;
        impl Monoid for Xor {
            type Val = usize;
            const E: Self::Val = 0;
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                left ^ right
            }
        }
        /// 最小値
        pub struct Min;
        impl Monoid for Min {
            type Val = isize;
            const E: Self::Val = (1 << 31) - 1;
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                *left.min(right)
            }
        }
        /// 最大値
        pub struct Max;
        impl Monoid for Max {
            type Val = isize;
            const E: Self::Val = -((1 << 31) - 1);
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                *left.max(right)
            }
        }
        /// 最小公倍数
        pub struct GCD;
        impl Monoid for GCD {
            type Val = usize;
            const E: Self::Val = 0;
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                gcd(*left, *right)
            }
        }
        pub fn gcd(a: usize, b: usize) -> usize {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
    }
}

mod segment_tree {
    //! セグメント木（モノイド）
    use crate::monoid::Monoid;
    use std::fmt;
    use std::ops::{
        Bound::{Excluded, Included, Unbounded},
        Deref, DerefMut, Index, RangeBounds,
    };
    /// # SegmentTree (Monoid)
    /// - 抽象化セグメント木
    pub struct SegmentTree<M: Monoid> {
        pub size: usize,
        offset: usize,
        data: Vec<M::Val>,
    }
    impl<M: Monoid> Index<usize> for SegmentTree<M> {
        type Output = M::Val;
        fn index(&self, idx: usize) -> &Self::Output {
            &self.data[self.offset + idx]
        }
    }
    impl<M: Monoid> SegmentTree<M> {
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
        /// セグメント木を初期化する
        pub fn new(n: usize) -> Self {
            let offset = n;
            Self {
                size: n,
                offset,
                data: vec![M::E; offset << 1],
            }
        }
        pub fn update(&mut self, index: usize, value: M::Val) {
            let mut i = index + self.offset;
            self.data[i] = value;
            while i > 1 {
                i >>= 1;
                let lch = i << 1;
                self.data[i] = M::op(&self.data[lch], &self.data[lch + 1]);
            }
        }
        /// 可変な参照を返す
        pub fn get_mut(&mut self, i: usize) -> Option<ValMut<'_, M>> {
            if i < self.offset {
                let default = self.index(i).clone();
                Some(ValMut {
                    segtree: self,
                    idx: i,
                    new_val: default,
                })
            } else {
                None
            }
        }
        /// 区間`range`の集約を行う
        pub fn get_range<R: RangeBounds<usize> + fmt::Debug>(&self, range: R) -> M::Val {
            let Some((start, end)) = self.parse_range(&range) else {
                panic!("The given range is wrong: {:?}", range);
            };
            // 値の取得
            let mut l = self.offset + start;
            let mut r = self.offset + end;
            let (mut res_l, mut res_r) = (M::E, M::E);
            while l < r {
                if l & 1 == 1 {
                    res_l = M::op(&res_l, &self.data[l]);
                    l += 1;
                }
                if r & 1 == 1 {
                    r -= 1;
                    res_r = M::op(&self.data[r], &res_r);
                }
                l >>= 1;
                r >>= 1;
            }
            M::op(&res_l, &res_r)
        }
    }
    impl<M: Monoid> From<&Vec<M::Val>> for SegmentTree<M> {
        fn from(src: &Vec<M::Val>) -> Self {
            let mut seg = Self::new(src.len());
            for (i, v) in src.iter().enumerate() {
                seg.data[seg.offset + i] = v.clone();
            }
            for i in (0..seg.offset).rev() {
                let lch = i << 1;
                seg.data[i] = M::op(&seg.data[lch], &seg.data[lch + 1]);
            }
            seg
        }
    }
    impl<M: Monoid> std::fmt::Debug for SegmentTree<M> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "SegmentTree {{ [").ok();
            for i in 0..self.size {
                if i + 1 < self.size {
                    write!(f, "{:?}, ", self.data[self.offset + i]).ok();
                } else {
                    write!(f, "{:?}", self.data[self.offset + i]).ok();
                }
            }
            write!(f, "] }}")
        }
    }
    pub struct ValMut<'a, M: 'a + Monoid> {
        segtree: &'a mut SegmentTree<M>,
        idx: usize,
        new_val: M::Val,
    }
    impl<M: Monoid> fmt::Debug for ValMut<'_, M> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("ValMut")
                .field(&self.segtree.index(self.idx))
                .finish()
        }
    }
    impl<M: Monoid> Drop for ValMut<'_, M> {
        fn drop(&mut self) {
            self.segtree.update(self.idx, self.new_val.clone());
        }
    }
    impl<M: Monoid> Deref for ValMut<'_, M> {
        type Target = M::Val;
        fn deref(&self) -> &Self::Target {
            &self.segtree[self.idx]
        }
    }
    impl<M: Monoid> DerefMut for ValMut<'_, M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.new_val
        }
    }
}

//      The Maximum Number of Overlaps
// ----------------------------------------
// 問題
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_5_B&lang=ja
// ----------------------------------------

use crate::get_ as get;
use monoid::examples::Add;
use segment_tree_2D::SegmentTree2D;

const MAX: usize = 1010;

fn main() {
    let N = get!(usize);

    let mut seg = SegmentTree2D::<Add>::new(MAX, MAX);

    for _ in 0..N {
        let (a, b, c, d) = get!(usize, usize, usize, usize);

        *seg.get_mut(a, b).unwrap() += 1;
        *seg.get_mut(a, d).unwrap() -= 1;
        *seg.get_mut(c, b).unwrap() -= 1;
        *seg.get_mut(c, d).unwrap() += 1;

        // seg.show();
    }

    let mut ans = 0;
    for i in 0..MAX {
        for j in 0..MAX {
            ans = ans.max(seg.get_range(0..i + 1, 0..j + 1));
        }
    }

    println!("{}", ans);
}

mod get_macro {
    //! 入力用マクロ
    //! - 参考：[Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
    /// 入力用マクロ
    #[macro_export]
    macro_rules! get_ {
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
        ($t:ty ; $n:expr) => {
            (0..$n).map(|_|
                get_!($t)
            ).collect::<Vec<_>>()
        };
        ($($t:ty),* ; $n:expr) => {
            (0..$n).map(|_|
                get_!($($t),*)
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
                get_!($t ;;)
            ).collect::<Vec<_>>()
        };
    }
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
        /// アフィン変換（浮動小数点数）
        struct Affine;
        impl Monoid for Affine {
            type Val = (f64, f64);
            const E: Self::Val = (1.0, 0.0);
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                let &(a1, b1) = left;
                let &(a2, b2) = right;
                (a2 * a1, a2 * b1 + b2)
            }
        }
    }
}

mod segment_tree_2D {
    //! セグメント木（二次元）
    //! - 参考：[二次元セグメント木 - Nyaan's Library](https://nyaannyaan.github.io/library/data-structure-2d/2d-segment-tree.hpp.html)
    /// cfor! {}
    macro_rules! cfor {
        ($def:stmt ; $fin:expr ; $incr:stmt ;; $bl:block) => {{
            $def
            while $fin {
                $bl
                $incr
            }
        }}
    }
    use super::monoid::Monoid;
    use std::fmt::{self, Debug};
    use std::ops::{
        Bound::{Excluded, Included, Unbounded},
        Deref, DerefMut, RangeBounds,
    };
    /// # SegmentTree2D (Monoid)
    /// - 2次元セグメント木
    pub struct SegmentTree2D<M: Monoid> {
        pub oh: usize,
        pub ow: usize,
        pub data: Vec<M::Val>,
    }
    impl<M: Monoid> SegmentTree2D<M> {
        #[inline]
        fn parse_range<R: RangeBounds<usize>>(
            &self,
            range: &R,
            max: usize,
        ) -> Option<(usize, usize)> {
            let start = match range.start_bound() {
                Unbounded => 0,
                Excluded(&v) => v + 1,
                Included(&v) => v,
            };
            let end = match range.end_bound() {
                Unbounded => max,
                Excluded(&v) => v,
                Included(&v) => v + 1,
            };
            if start <= end && end <= max {
                Some((start, end))
            } else {
                None
            }
        }
        #[inline]
        fn idx(&self, i: usize, j: usize) -> usize {
            2 * self.ow * i + j
        }
        /// セグメント木を初期化する
        pub fn new(H: usize, W: usize) -> Self {
            let oh = H.next_power_of_two();
            let ow = W.next_power_of_two();
            Self {
                oh,
                ow,
                data: vec![M::E; 4 * oh * ow],
            }
        }
        /// 座標 `(r,c)` の値を `x` に更新する
        pub fn update(&mut self, mut r: usize, mut c: usize, x: M::Val) {
            r += self.oh;
            c += self.ow;
            let idx = self.idx(r, c);
            self.data[idx] = x;
            // col方向の更新
            cfor! {let mut i = r >> 1; i > 0; i >>= 1;; {
                let idx = self.idx(i, c);
                self.data[idx] = M::op(
                    &self.data[self.idx(2 * i, c)],
                    &self.data[self.idx(2 * i + 1, c)],
                );
            }}
            // row方向の更新
            cfor! {let mut i = r; i > 0; i >>= 1;; {
                cfor! {let mut j = c >> 1; j > 0; j >>= 1;; {
                    let idx = self.idx(i, j);
                    self.data[idx] = M::op(
                        &self.data[self.idx(i, 2 * j)],
                        &self.data[self.idx(i, 2 * j + 1)],
                    );
                }}
            }}
        }
        /// 可変な参照を返す
        pub fn get_mut(&mut self, r: usize, c: usize) -> Option<ValMut<'_, M>> {
            if r < self.oh && c < self.ow {
                let old_val = self.data[self.idx(r + self.oh, c + self.ow)].clone();
                Some(ValMut {
                    segtree: self,
                    r,
                    c,
                    new_val: old_val,
                })
            } else {
                None
            }
        }
        /// row方向での集約を行う
        fn aggregate_row(&self, r: usize, mut cs: usize, mut ce: usize) -> M::Val {
            // 集約
            let mut res = M::E;
            while cs < ce {
                if cs & 1 == 1 {
                    res = M::op(&res, &self.data[self.idx(r, cs)]);
                    cs += 1;
                }
                if ce & 1 == 1 {
                    ce -= 1;
                    res = M::op(&res, &self.data[self.idx(r, ce)]);
                }
                cs >>= 1;
                ce >>= 1;
            }
            res
        }
        /// 区間の集約を行う
        pub fn get_range<R, C>(&self, row: R, col: C) -> M::Val
        where
            R: RangeBounds<usize> + fmt::Debug,
            C: RangeBounds<usize> + fmt::Debug,
        {
            let (mut rs, mut re) = {
                let res = self.parse_range(&row, self.oh);
                if res.is_none() {
                    panic!("The given range is wrong (row): {:?}", row);
                }
                res.unwrap()
            };
            let (mut cs, mut ce) = {
                let res = self.parse_range(&col, self.ow);
                if res.is_none() {
                    panic!("The given range is wrong (col): {:?}", col);
                }
                res.unwrap()
            };
            rs += self.oh;
            re += self.oh;
            cs += self.ow;
            ce += self.ow;
            // 値の取得
            let mut res = M::E;
            while rs < re {
                if rs & 1 == 1 {
                    res = M::op(&res, &self.aggregate_row(rs, cs, ce));
                    rs += 1;
                }
                if re & 1 == 1 {
                    re -= 1;
                    res = M::op(&res, &self.aggregate_row(re, cs, ce));
                }
                rs >>= 1;
                re >>= 1;
            }
            res
        }
    }
    impl<M: Monoid> From<&Vec<Vec<M::Val>>> for SegmentTree2D<M> {
        fn from(src: &Vec<Vec<M::Val>>) -> Self {
            let (H, W) = (src.len(), src[0].len());
            let mut seg = SegmentTree2D::new(H, W);
            let (oh, ow) = (seg.oh, seg.ow);
            // セグ木の値を埋める
            for i in 0..H {
                for j in 0..W {
                    let idx = seg.idx(oh + i, ow + j);
                    seg.data[idx] = src[i][j].clone();
                }
            }
            // col方向の集約
            for j in ow..2 * ow {
                for i in (1..oh).rev() {
                    let idx = seg.idx(i, j);
                    seg.data[idx] = M::op(
                        &seg.data[seg.idx(2 * i, j)],
                        &seg.data[seg.idx(2 * i + 1, j)],
                    );
                }
            }
            // row方向の集約
            for i in 0..2 * oh {
                for j in (1..ow).rev() {
                    let idx = seg.idx(i, j);
                    seg.data[idx] = M::op(
                        &seg.data[seg.idx(i, 2 * j)],
                        &seg.data[seg.idx(i, 2 * j + 1)],
                    );
                }
            }
            seg
        }
    }
    pub struct ValMut<'a, M: 'a + Monoid> {
        segtree: &'a mut SegmentTree2D<M>,
        r: usize,
        c: usize,
        new_val: M::Val,
    }
    impl<M: Monoid> fmt::Debug for ValMut<'_, M> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ValMut")
                .field("r", &self.r)
                .field("c", &self.c)
                .field("new_val", &self.new_val)
                .finish()
        }
    }
    impl<M: Monoid> Drop for ValMut<'_, M> {
        fn drop(&mut self) {
            self.segtree.update(self.r, self.c, self.new_val.clone());
        }
    }
    impl<M: Monoid> Deref for ValMut<'_, M> {
        type Target = M::Val;
        fn deref(&self) -> &Self::Target {
            &self.new_val
        }
    }
    impl<M: Monoid> DerefMut for ValMut<'_, M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.new_val
        }
    }
    impl<M> SegmentTree2D<M>
    where
        M: Monoid,
        M::Val: Debug,
    {
        /// セグ木を簡易的に表示する
        pub fn show(&self) {
            #![cfg(debug_assertions)]
            let H = self.oh;
            let W = self.ow;
            let idx = |r: usize, c: usize| -> usize { 2 * r * W + c };
            let mut r = 1;
            let mut h = 1;
            let mut logh = 0;
            while r + h <= 2 * H {
                for i in 1..=h {
                    let mut c = 1;
                    let mut w = 1;
                    while c + w <= 2 * W {
                        eprintln!(
                            "{}{:?}",
                            "  ".repeat(logh),
                            &self.data[idx(r + i - 1, c)..idx(r + i - 1, c + w)]
                        );
                        c += w;
                        w <<= 1;
                    }
                }
                r += h;
                h <<= 1;
                logh += 1;
            }
            eprintln!();
        }
    }
}

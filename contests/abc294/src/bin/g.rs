use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        UVW: [(Usize1, Usize1, isize); N - 1],
        Q: usize,
    }

    // 木の構築
    let G = UVW.iter().fold(vec![vec![]; N], |mut g, &(u, v, w)| {
        g[u].push((v, w));
        g[v].push((u, w));
        g
    });

    // オイラーツアー
    

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

mod monoid {
    #![allow(dead_code)]
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

mod segment_tree {
    #![allow(dead_code)]
    //! セグメント木
    use crate::monoid::Monoid;
    use std::fmt::{self, Debug};
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
        pub fn get_range<R: RangeBounds<usize> + Debug>(&self, range: R) -> M::Val {
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
    impl<M: Monoid> Debug for SegmentTree<M> {
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
    impl<M: Monoid> Debug for ValMut<'_, M> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("ValMut").field(&self.new_val).finish()
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
            &self.new_val
        }
    }
    impl<M: Monoid> DerefMut for ValMut<'_, M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.new_val
        }
    }
    impl<M> SegmentTree<M>
    where
        M: Monoid,
        M::Val: Debug,
    {
        /// セグ木を簡易的に表示する
        /// **サイズが2べきのときのみ**
        pub fn show(&self) {
            #![cfg(debug_assertions)]
            let mut i = 1;
            let mut w = 1;
            while i + w <= 2 * self.offset {
                eprintln!("{:?}", &self.data[i..i + w]);
                i += w;
                w <<= 1;
            }
            eprintln!();
        }
    }
}

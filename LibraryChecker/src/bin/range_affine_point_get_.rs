#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use crate::get_ as get;

use self::{
    lazy_segment_tree::{Alg::Affine1dMod, LazySegmentTree},
    modint::{Mod998, M998},
};

fn main() {
    let (N, Q) = get!(usize, usize);
    let A = get!(usize;;);

    let mut seg =
        LazySegmentTree::<Affine1dMod<M998>>::from(&A.iter().map(|&v| Mod998::new(v)).collect());

    for _ in 0..Q {
        let q = get!(String);
        let qq: Vec<usize> = q
            .split_ascii_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect();

        match qq[..] {
            [0, l, r, b, c] => {
                seg.apply(l..r, (b.into(), c.into()));
                if cfg!(debug_assertions) {
                    eprintln!("{}", seg.show());
                }
            }
            [1, i] => {
                println!("{}", seg.get(i..=i));
            }
            _ => (),
        }
    }
}

mod get_macro {
    //! 入力用マクロ
    // [Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
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

mod lazy_segment_tree {
    //! 遅延評価セグメント木
    use core::fmt;
    use std::{
        fmt::Debug,
        ops::{
            Bound::{Excluded, Included, Unbounded},
            RangeBounds,
        },
    };
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
    pub mod Alg {
        use super::ExtMonoid;
        use crate::modint::modint::Modint;
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
            const IX: Self::X = 1 << 31;
            fn operate_x(x: &Self::X, y: &Self::X) -> Self::X {
                *x.min(y)
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
        /// ## 1次元Affine変換
        /// - 区間を`ax + b`で更新（Affine変換）
        /// - 区間和を取得
        #[derive(Debug)]
        pub struct Affine1dMod<const MOD: usize>;
        impl<const MOD: usize> ExtMonoid for Affine1dMod<MOD> {
            type X = Modint<MOD>;
            type M = (Modint<MOD>, Modint<MOD>);
            const IX: Self::X = Modint::<MOD>(0);
            const IM: Self::M = (Modint::<MOD>(1), Modint::<MOD>(0));
            fn operate_x(x: &Self::X, y: &Self::X) -> Self::X {
                *x + *y
            }
            fn operate_m(x: &Self::M, y: &Self::M) -> Self::M {
                let &(a1, b1) = x;
                let &(a2, b2) = y;
                //   a2 * (a1 * x + b1) + b2
                // = (a2 * a1) * x + (a2 * b1 + b2)
                (a2 * a1, a2 * b1 + b2)
            }
            fn apply(x: &Self::X, y: &Self::M) -> Self::X {
                let &(a, b) = y;
                a * *x + b
            }
            fn aggregate(x: &Self::M, p: usize) -> Self::M {
                let &(a, b) = x;
                (a, b * p)
            }
        }
    }
}

mod modint {
    //! Modintの構造体
    use modint::*;
    pub const M998: usize = 998244353;
    pub const M109: usize = 1000000007;
    pub type Mod998 = Modint<M998>;
    pub type Mod109 = Modint<M109>;
    // 適当な素数
    pub type P1 = Modint<938472061>;
    pub type P2 = Modint<958472071>;
    #[rustfmt::skip]
    pub mod modint {
        use std::{fmt::Display,ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign}, str::FromStr, num::ParseIntError, iter::{Sum, Product}};
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)] pub struct Modint<const MOD: usize>(pub usize);
        impl<const MOD: usize> Modint<MOD> { pub fn new(n: usize) -> Self { Self(if n < MOD { n } else { n % MOD }) }
        pub fn from_str(s: &str) -> Self { s.chars().fold(0.into(), |n, c| n * 10 + c.to_digit(10).unwrap() as usize) } }
        impl<const MOD: usize> Neg for Modint<MOD> { type Output = Self; fn neg(self) -> Self { Modint(if self.0 == 0 { 0 } else { MOD - self.0 }) } }
        impl<const MOD: usize> Add for Modint<MOD> { type Output = Self; fn add(self, rhs: Self) -> Self { let mut res = self.0 + rhs.0; if res >= MOD { res -= MOD; } Modint(res) } }
        impl<const MOD: usize> Sub for Modint<MOD> { type Output = Self; fn sub(self, rhs: Self) -> Self { self + (- rhs) } }
        impl<const MOD: usize> Mul for Modint<MOD> { type Output = Self; fn mul(self, rhs: Self) -> Self { Modint(self.0 * rhs.0 % MOD) } }
        impl<const MOD: usize> Div for Modint<MOD> { type Output = Self; fn div(self, rhs: Self) -> Self { self * rhs.inv() } }
        impl<const MOD: usize> AddAssign for Modint<MOD> { fn add_assign(&mut self, rhs: Self) { self.0 = (*self + rhs).0 } }
        impl<const MOD: usize> SubAssign for Modint<MOD> { fn sub_assign(&mut self, rhs: Self) { self.0 = (*self - rhs).0 } }
        impl<const MOD: usize> MulAssign for Modint<MOD> { fn mul_assign(&mut self, rhs: Self) { self.0 = (*self * rhs).0 } }
        impl<const MOD: usize> From<usize> for Modint<MOD> { fn from(value: usize) -> Self { Modint::new(value) } }
        impl<const MOD: usize> Add<usize> for Modint<MOD> { type Output = Self; fn add(self, rhs: usize) -> Self { let mut res = self.0 + rhs; if res >= MOD {res -= MOD;} Modint(res) } }
        impl<const MOD: usize> Sub<usize> for Modint<MOD> { type Output = Self; fn sub(self, rhs: usize) -> Self { self - Modint::new(rhs) } }
        impl<const MOD: usize> Mul<usize> for Modint<MOD> { type Output = Self; fn mul(self, rhs: usize) -> Self { self * Modint::new(rhs) } }
        impl<const MOD: usize> Div<usize> for Modint<MOD> { type Output = Self; fn div(self, rhs: usize) -> Self { self / Modint::new(rhs) } }
        impl<const MOD: usize> AddAssign<usize> for Modint<MOD> { fn add_assign(&mut self, rhs: usize) { *self += Modint::new(rhs) } }
        impl<const MOD: usize> SubAssign<usize> for Modint<MOD> { fn sub_assign(&mut self, rhs: usize) { *self -= Modint::new(rhs) } }
        impl<const MOD: usize> MulAssign<usize> for Modint<MOD> { fn mul_assign(&mut self, rhs: usize) { *self *= Modint::new(rhs) } }
        impl<const MOD: usize> Display for Modint<MOD> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) } }
        impl<const MOD: usize> PartialEq<usize> for Modint<MOD> { fn eq(&self, other: &usize) -> bool { self == &Modint::new(*other) } }
        impl<const MOD: usize> FromStr for Modint<MOD> { type Err = ParseIntError; fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(Self::from_str(s)) } }
        pub trait Fp { fn pow(&self, rhs: usize) -> Self; fn inv(&self) -> Self; }
        impl<const MOD: usize> Fp for Modint<MOD> { fn pow(&self, rhs: usize) -> Self { let (mut a, mut b) = (self.0, rhs); let mut res = 1; while b > 0 { if b & 1 == 1 { res = (res * a) % MOD; } a = (a * a) % MOD; b >>= 1u32; } Modint(res) } fn inv(&self) -> Self { self.pow(MOD - 2) } }
        impl<const MOD: usize> Sum<Modint<MOD>> for Modint<MOD> { fn sum<I: Iterator<Item = Modint<MOD>>>(iter: I) -> Self { iter.fold(Modint::<MOD>(0), |acc, x| acc + x) } }
        impl<const MOD: usize> Product<Modint<MOD>> for Modint<MOD> { fn product<I: Iterator<Item = Modint<MOD>>>(iter: I) -> Self { iter.fold(Modint::<MOD>(1), |acc, x| acc * x) } }
    }
}

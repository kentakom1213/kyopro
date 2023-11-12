// https://mojacoder.app/users/shinnshinn/contests/ochacon02/tasks/4

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// [Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
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

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

#[rustfmt::skip]
pub mod modint {
    use std::{fmt::Display,ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign}, str::FromStr, num::ParseIntError};
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)] pub struct Modint<const MOD: usize>(pub usize);
    impl<const MOD: usize> Modint<MOD> { pub fn new(n: usize) -> Self { Self(if n < MOD { n } else { n % MOD }) } }
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
    impl<const MOD: usize> FromStr for Modint<MOD> { type Err = ParseIntError; fn from_str(s: &str) -> Result<Self, Self::Err> { usize::from_str(s).map(Modint::new) } }
    pub trait Fp { fn pow(&self, rhs: usize) -> Self; fn inv(&self) -> Self; }
    impl<const MOD: usize> Fp for Modint<MOD> { fn pow(&self, rhs: usize) -> Self { let (mut a, mut b) = (self.0, rhs); let mut res = 1; while b > 0 { if b & 1 == 1 { res = (res * a) % MOD; } a = (a * a) % MOD; b >>= 1u32; } Modint(res) } fn inv(&self) -> Self { self.pow(MOD - 2) } }
}
use modint::*;
pub type Mod998 = Modint<998244353>;
pub type Mod1e9 = Modint<1000000007>;

use std::ops::{
    Bound::{Excluded, Included, Unbounded},
    RangeBounds,
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
pub struct LazySegmentTree<T: ExtMonoid> {
    pub size: usize,
    offset: usize,
    data: Vec<T::X>,
    lazy: Vec<T::M>,
}
impl<T: ExtMonoid> LazySegmentTree<T> {
    #[inline]
    fn parse_range<R: RangeBounds<usize>>(&self, range: R) -> Option<(usize, usize)> {
        let start = match range.start_bound() {
            Unbounded => 0,
            Excluded(&v) => v + 1,
            Included(&v) => v,
        }
        .min(self.size);
        let end = match range.end_bound() {
            Unbounded => self.size,
            Excluded(&v) => v,
            Included(&v) => v + 1,
        }
        .min(self.size);
        if start <= end {
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
            data: vec![T::IX; offset << 1],
            lazy: vec![T::IM; offset << 1],
        }
    }
    /// 遅延値を評価
    fn eval(&mut self, idx: usize, len: usize) {
        if self.lazy[idx] == T::IM {
            return;
        }
        // 葉でなければ子に伝搬
        if idx < self.offset {
            self.lazy[idx * 2] = T::operate_m(&self.lazy[idx * 2], &self.lazy[idx]);
            self.lazy[idx * 2 + 1] = T::operate_m(&self.lazy[idx * 2 + 1], &self.lazy[idx]);
        }
        // 自身を更新
        self.data[idx] = T::apply(&self.data[idx], &T::aggregate(&self.lazy[idx], len));
        self.lazy[idx] = T::IM;
    }
    /// 区間に`val`を作用させる
    /// - `range`: `[left, right)`
    pub fn apply<R: RangeBounds<usize>>(&mut self, range: R, val: T::M) {
        if let Some((left, right)) = self.parse_range(range) {
            self.apply_inner(left, right, val, 0, self.offset, 1);
        }
    }
    fn apply_inner(
        &mut self,
        left: usize,
        right: usize,
        val: T::M,
        begin: usize,
        end: usize,
        idx: usize,
    ) {
        // 遅延値を評価
        self.eval(idx, end - begin);
        // 区間を内包するとき
        if left <= begin && end <= right {
            self.lazy[idx] = T::operate_m(&self.lazy[idx], &val);
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
            self.data[idx] = T::operate_x(&self.data[idx * 2], &self.data[idx * 2 + 1]);
        }
    }
    /// 区間を取得する
    /// - `range`: `[left, right)`
    pub fn get<R: RangeBounds<usize>>(&mut self, range: R) -> T::X {
        if let Some((left, right)) = self.parse_range(range) {
            self.get_inner(left, right, 0, self.offset, 1)
        } else {
            T::IX
        }
    }
    fn get_inner(
        &mut self,
        left: usize,
        right: usize,
        begin: usize,
        end: usize,
        idx: usize,
    ) -> T::X {
        // 遅延値を評価
        self.eval(idx, end - begin);
        // 区間を含まない
        if end <= left || right <= begin {
            T::IX
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
            T::operate_x(&l_val, &r_val)
        }
    }
}
impl<T: ExtMonoid> From<&Vec<T::X>> for LazySegmentTree<T> {
    fn from(src: &Vec<T::X>) -> Self {
        let mut seg = Self::new(src.len());
        for (i, v) in src.iter().enumerate() {
            seg.data[seg.offset + i] = v.clone();
        }
        for i in (0..seg.offset).rev() {
            let lch = i << 1;
            seg.data[i] = T::operate_x(&seg.data[lch], &seg.data[lch + 1]);
        }
        seg
    }
}
pub mod Alg {
    use super::ExtMonoid;
    use super::Modint;
    /// ## RSQandRAQ
    /// - 区間加算
    /// - 区間和
    #[derive(Debug)]
    pub struct RSQandRAQ;
    impl ExtMonoid for RSQandRAQ {
        type X = Modint<998244353>;
        type M = Modint<998244353>;
        const IX: Self::X = Modint::<998244353>(0);
        const IM: Self::M = Modint::<998244353>(0);
        fn operate_x(x: &Self::X, y: &Self::X) -> Self::X {
            *x + *y
        }
        fn apply(x: &Self::X, y: &Self::M) -> Self::X {
            *x + *y
        }
        fn operate_m(x: &Self::M, y: &Self::M) -> Self::M {
            *x + *y
        }
        fn aggregate(x: &Self::M, p: usize) -> Self::M {
            *x * p
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
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    let N = get!(usize);
    let mut arr = get!(Modint<998244353>;;);
    
    let mut sum = vec![Mod998::new(0) ; N + 1];
    for i in 0..N {
        sum[i + 1] = sum[i] + arr[i];
    }

    // 累積和のセグ木
    let mut S = LazySegmentTree::<Alg::RSQandRAQ>::from(&sum);

    debug!(&S);

    // クエリの処理
    let Q = get!(usize);
    for _ in 0..Q {
        let (t, a, b) = get!(usize, usize, usize);

        if t == 1 {
            // 累積和の累積和を計算
            let ans = S.get(a - 1..=b);
            println!("{}", ans);
        } else {
            // 区間加算
            let diff = Mod998::new(b) - arr[a - 1];
            S.apply(a - 1.., diff);
        }

        debug!(S);
    }
}

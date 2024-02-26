#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

use crate::{modint::Mint998, union_find::UnionFind};

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
        H: usize,
        W: usize,
        S: [Chars; H],
    }

    let idx = |r: usize, c: usize| -> usize { r * W + c };

    // 赤色のマスの個数
    let reds = {
        let mut r = vec![];
        for i in 0..H {
            for j in 0..W {
                if S[i][j] == '.' {
                    r.push((i, j));
                }
            }
        }
        r
    };

    debug!(reds);

    // ufを構築
    let mut uf = {
        let mut uf = UnionFind::new(H * W);
        for i in 0..H {
            for j in 0..W {
                // 下
                if i + 1 < H && S[i][j] == '#' && S[i + 1][j] == '#' {
                    uf.unite(idx(i, j), idx(i + 1, j));
                }
                // 右
                if j + 1 < W && S[i][j] == '#' && S[i][j + 1] == '#' {
                    uf.unite(idx(i, j), idx(i, j + 1));
                }
            }
        }
        uf
    };

    // グループの数
    let G = uf.group_count - reds.len();

    debug!(G);
    debug!(uf);

    // 期待値を求める
    let mut sum = Mint998::new(0);

    for &(i, j) in &reds {
        let mut adj = BTreeSet::new();

        // 上
        if i > 0 && S[i - 1][j] == '#' {
            let g = uf.root(idx(i - 1, j));
            adj.insert(g);
        }
        // 左
        if j > 0 && S[i][j - 1] == '#' {
            let g = uf.root(idx(i, j - 1));
            adj.insert(g);
        }
        // 下
        if i + 1 < H && S[i + 1][j] == '#' {
            let g = uf.root(idx(i + 1, j));
            adj.insert(g);
        }
        // 右
        if j + 1 < W && S[i][j + 1] == '#' {
            let g = uf.root(idx(i, j + 1));
            adj.insert(g);
        }
        debug!((i, j), adj);

        // 隣接するグループの総数
        sum += G - adj.len() + 1;
    }

    let ans = sum / reds.len();
    debug!(ans);
    println!("{ans}");
}

mod union_find {
    //! UnionFind木
    /// # UnionFind
    #[derive(Debug)]
    pub struct UnionFind {
        par: Vec<usize>,
        siz: Vec<usize>,
        /// 連結成分の個数
        pub group_count: usize,
    }
    impl UnionFind {
        /// UnionFindを新規作成
        pub fn new(n: usize) -> Self {
            UnionFind {
                par: (0..n).collect(),
                siz: vec![1; n],
                group_count: n,
            }
        }
        /// 根を求める
        pub fn root(&mut self, x: usize) -> usize {
            if self.par[x] == x {
                return x;
            }
            self.par[x] = self.root(self.par[x]); // 経路圧縮
            self.par[x]
        }
        /// 同一の集合に所属するか判定
        pub fn issame(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
        }
        /// 要素を結合
        pub fn unite(&mut self, mut parent: usize, mut child: usize) -> bool {
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
            self.group_count -= 1;
            true
        }
        /// 連結成分の大きさを求める
        pub fn size(&mut self, x: usize) -> usize {
            let root = self.root(x);
            self.siz[root]
        }
    }
}

mod modint {
    //! Modintの構造体
    pub use modint::*;
    pub const M998: usize = 998244353;
    pub const M107: usize = 1000000007;
    pub type Mint998 = Modint<M998>;
    pub type Mint107 = Modint<M107>;
    // 適当な素数
    pub type P1 = Modint<938472061>;
    pub type P2 = Modint<958472071>;
    #[rustfmt::skip]
    pub mod modint {
        fn sqrt(n: usize) -> usize { (n as f64).sqrt() as usize }
        use std::{fmt::{Debug, Display}, iter::{Sum, Product}, mem::replace, num::ParseIntError, ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign}, str::FromStr};
        #[derive(Clone, Copy, Default, PartialEq, Eq, Hash)] pub struct Modint<const MOD: usize>(pub usize);
        impl<const MOD: usize> Modint<MOD> { pub fn new(n: usize) -> Self { Self(if n < MOD { n } else { n % MOD }) }
        pub fn from_str(s: &str) -> Self { s.chars().fold(0.into(), |n, c| n * 10 + c.to_digit(10).unwrap() as usize) }
        pub fn from_isize(n: isize) -> Self { Self::new((MOD as isize + n % MOD as isize) as usize) }
        pub fn rational_reconstruction(&self) -> Option<(isize, isize)> { let N = sqrt(MOD / 2) as isize; let mut v = (MOD as isize, 0); let mut w = (self.0 as isize, 1);
        while w.0 > N { let q = v.0.div_euclid(w.0); let z = (v.0 - q * w.0, v.1 - q * w.1); v = replace(&mut w, z); } if w.1 < 0 { w = (-w.0, -w.1); } (w.0 <= N && w.1 <= N).then_some(w) } }
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
        impl<const MOD: usize> Debug for Modint<MOD> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { match self.rational_reconstruction() { Some((n, d)) => if d > 1 { write!(f, "Modint({n}/{d})") } else { write!(f, "Modint({n})") } _ => write!(f, "Modint({})", self.0) } } }
        pub trait Fp { fn pow(&self, rhs: usize) -> Self; fn inv(&self) -> Self; }
        impl<const MOD: usize> Fp for Modint<MOD> { fn pow(&self, rhs: usize) -> Self { let (mut a, mut b) = (self.0, rhs); let mut res = 1; while b > 0 { if b & 1 == 1 { res = (res * a) % MOD; } a = (a * a) % MOD; b >>= 1u32; } Modint(res) } fn inv(&self) -> Self { self.pow(MOD - 2) } }
        impl<const MOD: usize> Sum<Modint<MOD>> for Modint<MOD> { fn sum<I: Iterator<Item = Modint<MOD>>>(iter: I) -> Self { iter.fold(Modint::<MOD>(0), |acc, x| acc + x) } }
        impl<const MOD: usize> Product<Modint<MOD>> for Modint<MOD> { fn product<I: Iterator<Item = Modint<MOD>>>(iter: I) -> Self { iter.fold(Modint::<MOD>(1), |acc, x| acc * x) } }
    }
}

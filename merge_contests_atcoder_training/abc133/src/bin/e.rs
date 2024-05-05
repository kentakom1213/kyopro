// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

#[rustfmt::skip]
pub mod modint {
    use std::{fmt::Display,ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub}, str::FromStr, num::ParseIntError};
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)] pub struct Modint<const MOD: usize>(pub usize);
    impl<const MOD: usize> Modint<MOD> { pub fn new(n: usize) -> Self { Self(if n < MOD { n } else { n % MOD }) } }
    impl<const MOD: usize> Neg for Modint<MOD> { type Output = Self; fn neg(self) -> Self { Modint(if self.0 == 0 { 0 } else { MOD - self.0 }) } }
    impl<const MOD: usize> Add for Modint<MOD> { type Output = Self; fn add(self, rhs: Self) -> Self { let mut res = self.0 + rhs.0; if res >= MOD { res -= MOD; } Modint(res) } }
    impl<const MOD: usize> Sub for Modint<MOD> { type Output = Self; fn sub(self, rhs: Self) -> Self { self + (- rhs) } }
    impl<const MOD: usize> Mul for Modint<MOD> { type Output = Self; fn mul(self, rhs: Self) -> Self { Modint(self.0 * rhs.0 % MOD) } }
    impl<const MOD: usize> Div for Modint<MOD> { type Output = Self; fn div(self, rhs: Self) -> Self { self * rhs.inv() } }
    impl<const MOD: usize> AddAssign for Modint<MOD> { fn add_assign(&mut self, rhs: Self) { self.0 = (*self + rhs).0 } }
    impl<const MOD: usize> MulAssign for Modint<MOD> { fn mul_assign(&mut self, rhs: Self) { self.0 = (*self * rhs).0 } }
    impl<const MOD: usize> From<usize> for Modint<MOD> { fn from(value: usize) -> Self { Modint::new(value) } }
    impl<const MOD: usize> Add<usize> for Modint<MOD> { type Output = Self; fn add(self, rhs: usize) -> Self { let mut res = self.0 + rhs; if res >= MOD {res -= MOD;} Modint(res) } }
    impl<const MOD: usize> Sub<usize> for Modint<MOD> { type Output = Self; fn sub(self, rhs: usize) -> Self { self - Modint::new(rhs) } }
    impl<const MOD: usize> Mul<usize> for Modint<MOD> { type Output = Self; fn mul(self, rhs: usize) -> Self { self * Modint::new(rhs) } }
    impl<const MOD: usize> Div<usize> for Modint<MOD> { type Output = Self; fn div(self, rhs: usize) -> Self { self / Modint::new(rhs) } }
    impl<const MOD: usize> AddAssign<usize> for Modint<MOD> { fn add_assign(&mut self, rhs: usize) { *self += Modint::new(rhs) } }
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

/// 二項係数を高速に求める
/// - 前計算: `O(N)`
/// - クエリ: `O(1)`
pub struct Comb<const MOD: usize> {
    fac: Vec<Modint<MOD>>,
    finv: Vec<Modint<MOD>>,
}
impl<const MOD: usize> Comb<MOD> {
    /// サイズ`max_size`で配列を初期化する
    pub fn new(max_size: usize) -> Self {
        let mod1: Modint<MOD> = 1.into();
        let mut fac = vec![mod1; max_size];
        let mut finv = vec![mod1; max_size];
        let mut inv = vec![mod1; max_size];
        for i in 2..max_size {
            fac[i] = fac[i - 1] * i;
            inv[i] = -Modint::new(MOD / i) * inv[MOD % i];
            finv[i] = finv[i - 1] * inv[i];
        }
        Comb { fac, finv }
    }
    /// 順列を求める
    pub fn comb(&self, n: usize, r: usize) -> Modint<MOD> {
        if n < r {
            return 0.into();
        }
        self.fac[n] * self.finv[r] * self.finv[n - r]
    }
    /// 組合せを求める
    pub fn perm(&self, n: usize, r: usize) -> Modint<MOD> {
        if n < r {
            return 0.into();
        }
        self.fac[n] * self.finv[n - r]
    }
    /// 重複を許す組合せ(combination with repetition)
    pub fn comb_with_rep(&self, n: usize, r: usize) -> Modint<MOD> {
        self.comb(n + r - 1, r)
    }
}

const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        K: usize,
        edges: [(Usize1, Usize1); N - 1]
    }

    let cmb = Comb::new(101010);

    // グラフの構築
    let mut G = vec![vec![]; N];

    for &(u, v) in &edges {
        G[u].push(v);
        G[v].push(u);
    }

    // DFSで塗り分け
    let mut ans = Mod1e9::new(1);

    // (自分, 親)
    let mut st = vec![(0, INF)];
    let mut is_visited = vec![false; N];
    is_visited[0] = true;

    while let Some((u, p)) = st.pop() {
        if p == INF {
            // 根のとき：自分の塗り方
            ans *= cmb.perm(K, 1);
            // 子の塗り方
            let c = G[u].len();
            ans *= cmb.perm(K.saturating_sub(1), c);
        } else {
            // 子の塗り分け方
            let c = G[u].len() - 1;
            ans *= cmb.perm(K.saturating_sub(2), c);
        }
        // 子を調べる
        for &v in &G[u] {
            if is_visited[v] {
                continue;
            }
            st.push((v, u));
            is_visited[v] = true;
        }
    }

    println!("{}", ans);
}

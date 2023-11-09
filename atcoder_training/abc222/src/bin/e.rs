// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::{collections::{HashMap, VecDeque}, mem::swap};

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

const INF: usize = 1001001001001001001;

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

fn main() {
    input! {
        N: usize,
        M: usize,
        K: isize,
        A: [Usize1; M],
        edges: [(Usize1, Usize1); N - 1]
    }

    // 各辺の番号
    let mut edge_id = HashMap::new();
    for (i, &(u, v)) in edges.iter().enumerate() {
        edge_id.insert((u, v), i);
        edge_id.insert((v, u), i);
    }

    // グラフ
    let mut G = vec![vec![]; N];
    for &(u, v) in &edges {
        G[u].push(v);
        G[v].push(u);
    }

    // ルートを調べ、各辺を通る回数を数え上げる
    let mut C = vec![0; N - 1];
    for i in 1..M {
        let start = A[i - 1];
        let end = A[i];
        count_edge(start, end, &G, &mut C, &edge_id);
    }

    debug!(&C);

    // 部分和問題として解く
    let OFFSET = 101010;
    let mut dp = vec![Mod998::new(0); OFFSET * 2];
    let mut nxt = vec![Mod998::new(0); OFFSET * 2];
    dp[OFFSET] += 1;

    for (i, &c) in C.iter().enumerate() {
        for j in 0..2 * OFFSET {
            if dp[j] == 0 {
                continue;
            }
            // -1倍して足す
            if j >= c && j - c < 2 * OFFSET {
                nxt[j - c] += dp[j];
            }
            // 1倍して足す
            if j + c < 2 * OFFSET {
                nxt[j + c] += dp[j];
            }
        }
        swap(&mut dp, &mut nxt);
        nxt.fill(Mod998::new(0));
    }

    // 総和がKになるもの
    let ans = dp[(OFFSET as isize + K) as usize];
    println!("{}", ans);
}

fn count_edge(start: usize, end: usize, G: &Vec<Vec<usize>>, C: &mut Vec<usize>, edge_id: &HashMap<(usize, usize), usize>) {
    let N = G.len();
    // dfsでルートを探索
    let mut q = VecDeque::new();
    q.push_back(start);
    let mut visited = vec![INF; N];
    visited[start] = 0;

    while let Some(u) = q.pop_front() {
        for &v in &G[u] {
            if visited[v] != INF {
                continue;
            }
            visited[v] = visited[u] + 1;
            q.push_back(v);
        }
    }

    // ルートの復元
    let mut route = vec![end];
    let mut cur = end;
    while cur != start {
        for &nxt in &G[cur] {
            if visited[nxt] + 1 == visited[cur] {
                route.push(nxt);
                cur = nxt;
            }
        }
    }

    debug!(end, start, &route);

    // 加算
    for i in 1..route.len() {
        let u = route[i - 1];
        let v = route[i];
        let e = edge_id[&(u, v)];
        C[e] += 1;
    }
}

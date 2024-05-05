//           E - Colorful Blocks
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc167/tasks/abc167_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
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

/// ## Fp
/// 有限体の実装
pub trait Fp {
    fn val(&self) -> usize;
    fn madd(&self, other: usize) -> usize;
    fn mneg(&self) -> usize;
    fn msub(&self, other: usize) -> usize;
    fn mmul(&self, other: usize) -> usize;
    fn minv(&self) -> usize;
    fn mdiv(&self, other: usize) -> usize;
    fn mpow(&self, other: usize) -> usize;
}

impl Fp for usize {
    fn val(&self) -> usize {
        self % MOD
    }

    fn madd(&self, other: usize) -> usize {
        (self.val() + other.val()).val()
    }

    fn mneg(&self) -> usize {
        (MOD - self.val()).val()
    }

    fn msub(&self, other: usize) -> usize {
        self.madd(other.mneg())
    }

    fn mmul(&self, other: usize) -> usize {
        (self.val() * other.val()).val()
    }

    fn mpow(&self, other: usize) -> usize {
        let (mut a, mut b) = (self.val(), other);
        let mut res = 1;
        while b > 0 {
            if b & 1 == 1 {
                res = res.mmul(a);
            }
            a = a.mmul(a);
            b >>= 1;
        }
        res
    }

    fn minv(&self) -> usize {
        assert!(*self != 0);
        self.mpow(MOD - 2)
    }

    fn mdiv(&self, other: usize) -> usize {
        self.mmul(other.minv())
    }
}

macro_rules! madd {
    ( $a:expr, $b:expr ) => {{
        let tmp = ($a).madd($b);
        $a = tmp;
    }};
}

/// # Comb
/// 二項係数を高速に求める
/// - 前計算: `O(N)`
/// - クエリ: `O(1)`
struct Comb {
    fac: Vec<usize>,
    finv: Vec<usize>,
}

impl Comb {
    fn new(max_size: usize) -> Self {
        let mut fac = vec![1; max_size];
        let mut finv = vec![1; max_size];
        let mut inv = vec![1; max_size];
        for i in 2..max_size {
            fac[i] = fac[i - 1] * i % MOD;
            inv[i] = MOD - (MOD / i) * inv[MOD % i] % MOD;
            finv[i] = finv[i - 1] * inv[i] % MOD;
        }

        Comb { fac, finv }
    }

    fn comb(&self, n: usize, r: usize) -> usize {
        if n < r {
            return 0;
        }
        self.fac[n] * (self.finv[r] * self.finv[n - r] % MOD) % MOD
    }

    fn perm(&self, n: usize, r: usize) -> usize {
        if n < r {
            return 0;
        }
        self.fac[n] * self.finv[n - r] % MOD
    }
}

const MOD: usize = 998244353;

// main
fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
    }

    let comb = Comb::new(1_000_000);

    let mut ans = 0;

    for k in 0..=K {
        let mut tmp = M;
        tmp = tmp.mmul(comb.comb(N - 1, k));
        tmp = tmp.mmul((M - 1).mpow(N - 1 - k));
        madd!(ans, tmp);
    }

    println!("{}", ans);
}

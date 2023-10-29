// E - Revenge of "The Salary of AtCoder Inc."
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc326/tasks/abc326_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![warn(clippy::default_numeric_fallback)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Usize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

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

pub trait FpAssign {
    fn madd_assign(&mut self, other: usize);
    fn msub_assign(&mut self, other: usize);
    fn mmul_assign(&mut self, other: usize);
}

impl FpAssign for usize {
    fn madd_assign(&mut self, other: usize) {
        *self = self.madd(other);
    }
    fn mmul_assign(&mut self, other: usize) {
        *self = self.mmul(other);
    }
    fn msub_assign(&mut self, other: usize) {
        *self = self.msub(other);
    }
}

const MOD: usize = 998244353;

fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut P = vec![0; N + 1];
    P[0] = 1;
    let mut sum = 1;

    let mut ans = 0;

    for i in 1..=N {
        P[i] = sum.mdiv(N);
        // 和を取る
        ans.madd_assign(P[i].mmul(A[i - 1]));
        // 累積和
        sum.madd_assign(P[i]);
    }

    debug!(&P);

    println!("{}", ans);
}

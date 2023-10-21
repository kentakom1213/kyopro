//               E - Roaming               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc156/tasks/abc156_e
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

// constant
const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        k: usize
    }

    const SIZE: usize = 200200;

    // 階乗の前計算
    let fact = {
        let mut arr = vec![1; SIZE];
        for i in 1..SIZE {
            arr[i] = arr[i - 1].mmul(i);
        }
        arr
    };

    // 組合せ
    let C = |n: usize, k: usize| -> usize {
        fact[n].mdiv(fact[k].mmul(fact[n - k]))
    };

    // 重複組合せ
    let H = |n: usize, k: usize| -> usize {
        C(n + k - 1, k)
    };

    // - 総和がn，0である要素がk個以下であるような
    //   長さnの非負数列の種類を数え上げる．
    // - 0である要素がi個であるとき，
    //   - 0である項を選ぶ組合せ → nCi
    //   - 総和がnになるような正数列
    //     → (n-i)種類のものから，重複を許して(n-(n-i))個選ぶ方法
    //     → (n-i)Hi
    let mut ans = 0;

    for i in 0..=k.min(n - 1) {
        madd!(
            ans,
            C(n, i).mmul(H(n-i, i))
        );
    }

    println!("{}", ans);
}

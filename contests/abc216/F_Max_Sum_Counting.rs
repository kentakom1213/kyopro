//           F - Max Sum Counting
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc216/tasks/abc216_f
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

// constant
const MOD: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// main
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        B: [usize; N],
    }

    // Aの値でソート
    let (A, B): (Vec<usize>, Vec<usize>) = A.iter().zip(B.iter()).sorted().unzip();

    debug!(&A);
    debug!(&B);

    const MAX: usize = 5050;

    // dp[i][j] := Bのi番目までの部分列において、その和がjとなるものの数
    let mut dp = vec![vec![0; MAX]; N + 1];
    dp[0][0] = 1;

    let mut ans = 0;

    for i in 0..N {
        // A[i]に対しての答えを求める
        if A[i] >= B[i] {
            let tmp = dp[i][..=A[i] - B[i]].iter().fold(0, |a, &b| a.madd(b));
            debug!(A[i].saturating_sub(B[i]), i, tmp);
            madd!(ans, tmp);
        }

        for j in 0..MAX {
            // dpを計算
            madd!(dp[i + 1][j], dp[i][j]);
            if j + B[i] < MAX {
                madd!(dp[i + 1][j + B[i]], dp[i][j]);
            }
        }
    }

    if cfg!(debug_assertions) {
        for r in &dp {
            println!("{:?}", r);
        }
    }

    println!("{}", ans);
}

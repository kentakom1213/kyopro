//           E - Unfair Sugoroku
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc298/tasks/abc298_e
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

/// ## Modint
/// 有限体の実装
pub trait Modint {
    fn val(&self) -> usize;
    fn madd(&self, other: usize) -> usize;
    fn mneg(&self) -> usize;
    fn msub(&self, other: usize) -> usize;
    fn mmul(&self, other: usize) -> usize;
    fn minv(&self) -> usize;
    fn mdiv(&self, other: usize) -> usize;
    fn mpow(&self, other: usize) -> usize;
}

impl Modint for usize {
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

macro_rules! madd_assign {
    ( $a:expr, $b:expr $(,)* ) => {{
        let tmp = ($a).madd($b);
        $a = tmp;
    }};
}

const INF: usize = 1001001001001001001;
const MOD: usize = 998_244_353;

// main
fn main() {
    input! {
        N: usize,
        A: usize,
        B: usize,
        P: usize,
        Q: usize,
    }

    let mut dp = vec![vec![vec![0; 2]; N + 1]; N + 1];

    for i in 0..N {
        for f in 0..2 {
            dp[N][i][f] = 1;
            dp[i][N][f] = 0;
        }
    }

    for i in (0..N).rev() {
        for j in (0..N).rev() {
            for k in 1..=P {
                madd_assign!(dp[i][j][0], dp[N.min(i + k)][j][1]);
            }
            dp[i][j][0] = dp[i][j][0].mdiv(P);
            for k in 1..=Q {
                madd_assign!(dp[i][j][1], dp[i][N.min(j + k)][0]);
            }
            dp[i][j][1] = dp[i][j][1].mdiv(Q);
        }
    }

    println!("{}", dp[A][B][0].val());
}

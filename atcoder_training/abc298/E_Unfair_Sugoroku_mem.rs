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

const MOD: usize = 998_244_353;

// main
fn main() {
    input! {
        N: usize,
        A: Usize1,
        B: Usize1,
        P: usize,
        Q: usize,
    }

    let mut dp: Vec<Vec<Vec<Option<usize>>>> = vec![vec![vec![None; N]; N]; 2];
    dp[0][N-1][N-1] = Some(1);
    dp[1][N-1][N-1] = Some(0);

    let ans = calc(A, B, 0, P, Q, N, &mut dp);

    println!("{}", ans);
}

fn calc(i: usize, j: usize, t: usize, P: usize, Q: usize, N: usize, dp: &mut Vec<Vec<Vec<Option<usize>>>>) -> usize {
    if let Some(res) = dp[t][i][j] {
        return res;
    }
    if t == 0 {
        let mut res = 0;
        for k in 1..=P {
            madd_assign!(res, calc((i+k).min(N-1), j, 1, P, Q, N, dp).mdiv(P));
        }
        dp[t][i][j] = Some(res);
        res
    }
    else {
        let mut res = 0;
        for k in 1..=Q {
            madd_assign!(res, calc(i, (j+k).min(N-1), 0, P, Q, N, dp).mdiv(Q));
        }
        dp[t][i][j] = Some(res);
        res
    }
}

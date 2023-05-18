//        E - Transformable Teacher
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc181/tasks/abc181_e
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

pub trait UsizeTools {
    fn abs_diff(&self, other: Self) -> Self;
    fn sqrt(&self) -> Self;
}

impl UsizeTools for usize {
    fn abs_diff(&self, other: Self) -> Self {
        if *self > other {
            *self - other
        } else {
            other - *self
        }
    }

    /// ## sqrt
    /// x^2がNを超えない最大のxを求める
    /// - 計算量：O(log(N))
    fn sqrt(&self) -> Self {
        let (mut ok, mut ng) = (0_usize, 1001001001001001001);
        while (ng - ok) > 1 {
            let m = (ok + ng) / 2;
            if m.saturating_mul(m) <= *self {
                ok = m;
            } else {
                ng = m;
            }
        }
        ok
    }
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// main
fn main() {
    input! {
        N: usize,
        M: usize,
        H: [usize; N],
        W: [usize; M],
    }

    // sum[i] := 左からi//2番目までをペアにしていったときの差の合計
    let mut sum = vec![0; N];
    for i in 0..N - 1 {
        if i % 2 == 0 {
            sum[i + 1] = sum[i];
        } else {
            sum[i + 1] = sum[i] + H[i + 1].abs_diff(H[i]);
        }
    }

    debug!(&sum);
}

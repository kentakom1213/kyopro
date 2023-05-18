//                D - M<=ab                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc296/tasks/abc296_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

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
    }

    // a <= b <= Nと固定する
    // このときab <= b^2
    // ab >= Mを求めたいので，b^2 <= M すなわち b <= √M まで調べる．
    // aの値は M/bであり，これがN以下の整数になるかを調べれば良い
    let mut ans = INF;

    for b in 1..=N.min(M.sqrt() + 10) {
        let a = (M + b - 1) / b;
        debug!(b, a);
        if a <= N {
            ans = ans.min(a * b);
        }
    }

    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

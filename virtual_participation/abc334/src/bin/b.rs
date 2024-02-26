#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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

macro_rules! debug2D {
    ( $array:expr ) => {{
        #![cfg(debug_assertions)]
        eprintln!("{}: ", stringify!($array));
        for row in &$array {
            eprintln!("{:?}", row);
        }
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        mut A: isize,
        M: isize,
        L: isize,
        R: isize,
    }

    A %= M;

    let MAX = 1001001001001001001;
    let MIN = -1001001001001001001;

    // x <= A+Mk を満たす最小のk
    let le = |x: isize| -> isize {
        let mut ng = MIN;
        let mut ok = MAX;
        while ok - ng > 1 {
            let mid = (ng + ok) / 2;
            if x - A <= M.saturating_mul(mid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    };

    // x < A+Mk を満たす最小のk
    let lt = |x: isize| -> isize {
        let mut ng = MIN;
        let mut ok = MAX;
        while ok - ng > 1 {
            let mid = (ng + ok) / 2;
            if x - A < M.saturating_mul(mid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    };

    let left = le(L);
    let right = lt(R);

    println!("{}", right - left);
}

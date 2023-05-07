// https://atcoder.jp/contests/arc126/tasks/arc126_b

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::cmp::Reverse;

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1, Isize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

fn debug_2d<T: std::fmt::Debug>(array: &Vec<Vec<T>>) {
    #[cfg(debug_assertions)]
    for row in array {
        println!("{:?}", row);
    }
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// # 重ならないか判定
/// - 端点の差が同符号
fn not_collided(x: (isize, isize), y: (isize, isize)) -> bool {
    (x.0 - y.0) * (x.1 - y.1) > 0
}

// main
fn main() {
    input! {
        N: usize,
        M: usize,
        mut L: [(isize, isize); M],
    }

    // aでソート
    L.sort_by_key(|&(a, b)| (a, b));
    debug!(&L);

    // 貪欲にとる
    let mut max_by_a = 0;
    let mut cur = (0, 0);
    for i in 0..M {
        if not_collided(cur, L[i]) {
            max_by_a += 1;
            cur = L[i];
        }
    }
    debug!(max_by_a);

    // bでソート
    L.sort_by_key(|&(a, b)| (b, a));
    debug!(&L);

    let mut max_by_b = 0;
    let mut cur = (0, 0);
    for i in 0..M {
        if not_collided(cur, L[i]) {
            max_by_b += 1;
            cur = L[i];
        }
    }

    println!("{}", max_by_a.max(max_by_b));
}
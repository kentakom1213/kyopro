// https://atcoder.jp/contests/abc107/tasks/arc101_a

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

fn debug_2d<T: std::fmt::Debug>(array: &Vec<Vec<T>>) {
    #[cfg(debug_assertions)]
    for row in array {
        println!("{:?}", row);
    }
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: isize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// main
fn main() {
    input! {
        N: usize,
        K: usize,
        X: [isize; N],
    }

    let mut ans = INF;

    // 最初にi個目の蝋燭を訪れるときの移動距離の最小値
    for i in 0..N - K + 1 {
        let l = X[i];
        let r = X[i + K - 1];
        debug!(l, r);

        ans = ans.min(
            (l - r).abs() + l.abs().min(r.abs())
        );
    }

    println!("{}", ans);
}

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::HashMap;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use rustc_hash::FxHashMap;

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
        N: usize,
        A: usize,
        X: f64,
        Y: f64,
    }

    let mut memo = HashMap::default();

    let ans = f(N, A, X, Y, &mut memo);

    println!("{ans:.10}");
}

fn f(N: usize, A: usize, X: f64, Y: f64, memo: &mut HashMap<usize, f64>) -> f64 {
    if N == 0 {
        return 0.0;
    }
    if let Some(&ans) = memo.get(&N) {
        return ans;
    }

    // X円払う場合
    let exX = X + f(N / A, A, X, Y, memo);

    // Y円払う場合
    let exY = 0.2 * (Y + (2..=6).map(|i| Y + f(N / i, A, X, Y, memo)).sum::<f64>());

    let ans = exX.min(exY);
    memo.insert(N, ans);
    ans
}

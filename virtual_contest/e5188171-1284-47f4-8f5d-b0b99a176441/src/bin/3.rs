// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::HashMap;

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
    }

    let mut memo = HashMap::<(usize, usize, usize), f64>::new();
    let ans = rec(A, B, C, &mut memo);

    println!("{:.20}", ans);
}

fn rec(a: usize, b: usize, c: usize, memo: &mut HashMap<(usize, usize, usize), f64>) -> f64 {
    // debug!(a, b, c);
    if let Some(&res) = memo.get(&(a, b, c)) {
        return res;
    }
    if a == 100 || b == 100 || c == 100 {
        return 0.;
    }
    // それぞれの確率
    let sum = (a + b + c) as f64;
    let res = a as f64 / sum * rec(a + 1, b, c, memo)
        + b as f64 / sum * rec(a, b + 1, c, memo)
        + c as f64 / sum * rec(a, b, c + 1, memo)
        + 1.0;
    memo.insert((a, b, c), res);
    res
}

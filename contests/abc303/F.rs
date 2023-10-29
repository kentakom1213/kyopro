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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// main
fn main() {
    input! {
        N: usize,
        H: usize,
        skill: [(usize, usize); N],
    }

    // ダメージの合計
    let total = skill
        .iter()
        .map(|&(t, d)| (t * d, t))
        .sorted()
        .rev()
        .collect_vec();

    debug!(&total);

    // 呪文のコスパ
    let perf = skill
        .iter()
        .enumerate()
        .map(|(i, &(t, d))| {
            let ft = t as f64;
            let fd = d as f64;
            let perf = fd / ft;
            (perf, i)
        })
        .sorted_by(|a, b| a.partial_cmp(&b).unwrap())
        .rev()
        .collect_vec();

    debug!(&perf);
}

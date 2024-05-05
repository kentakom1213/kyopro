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
        T: usize,
        M: usize,
        AB: [(Usize1, Usize1); M],
    }

    for r in combinations(N, T, N, T, &AB) {
        debug!(r);
    }
}

/// i番目の選手をj個目のチームに配置する
fn combinations(
    i: usize,
    j: usize,
    N: usize,
    T: usize,
    AB: &Vec<(usize, usize)>,
) -> Vec<Vec<Vec<usize>>> {
    if i == 0 || j == 0 {
        return vec![vec![]];
    }
    let mut res = vec![];
    let prv = combinations(i - 1, j - 1, N, T, AB);
    for g in prv {
        for m in i + 1..N {
            let mut tmp = g.clone();
            tmp.push((i..m).collect());
            res.push(tmp);
        }
    }
    res
}

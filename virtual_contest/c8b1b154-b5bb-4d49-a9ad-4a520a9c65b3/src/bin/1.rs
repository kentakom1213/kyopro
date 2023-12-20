// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::{collections::{BTreeMap, HashMap}, fmt::format};

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}};

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
        N: usize,
        M: usize,
        PY: [(usize, usize); M]
    }

    let mut ID = vec![vec![]; N + 1];

    for &(p, y) in &PY {
        ID[p].push(y);
    }

    let mut ans = HashMap::new();

    for i in 1..=N {
        ID[i].sort();

        for (j, &y) in ID[i].iter().enumerate() {
            ans.insert((i, y), format!("{:0>6}{:0>6}", i, j + 1));
        }
    }

    debug!(ID);
    debug!(ans);

    for x in PY {
        println!("{}", ans[&x]);
    }
}

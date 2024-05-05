// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::BTreeMap;
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

fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    // Aの総xorが0でないとき，max Ai 以上の任意のkで必勝
    if A.iter().fold(0, |acc, x| acc ^ x) != 0 {
        println!("-1");
        return;
    }

    // Aiの値をカウントしていく
    let C = A.iter().fold(BTreeMap::new(), |mut map, &v| {
        *map.entry(v).or_insert(0) += 1;
        map
    });

    debug!(C);

    // Cvが奇数となる最大のv
    let M = C.iter().rev().find(|&(v, cv)| cv % 2 == 1);

    if let Some((&M, _)) = M {
        println!("{}", M - 1);
    } else {
        println!("0");
    }
}

const INF: usize = 1001001001001001001;

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::BTreeSet;

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

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
        boxes: [(usize, usize, usize); N],
    }

    // 座標圧縮
    let mut comp = vec![];
    for &(x, y, z) in &boxes {
        comp.push(x);
        comp.push(y);
        comp.push(z);
    }
    comp.sort();
    comp.dedup();

    debug!(&comp);

    // let mut set = BTreeSet::new();
    // for &(x, y, z) in &boxes {
    //     set.insert((x, y, z));
    //     set.insert((x, z, y));
    //     set.insert((y, x, z));
    //     set.insert((y, z, x));
    //     set.insert((z, x, y));
    //     set.insert((z, y, z));
    // }


}

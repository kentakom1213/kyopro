#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::{BTreeMap, BTreeSet};

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

#[fastout]
fn main() {
    input! {
        N: usize,
        T: usize,
        AB: [(Usize1, usize); T]
    }

    let mut score = vec![0; N];
    let mut map = BTreeMap::new();

    map.insert(0, N);

    for &(a, b) in &AB {
        *map.get_mut(&score[a]).unwrap() -= 1;

        if map[&score[a]] == 0 {
            map.remove(&score[a]);
        }

        score[a] += b;

        *map.entry(score[a]).or_insert(0) += 1;

        println!("{}", map.len());
    }
}

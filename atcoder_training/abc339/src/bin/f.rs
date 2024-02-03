#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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

use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};
use rustc_hash::FxHashMap;

/// 適当な素数
const PRIMES: [usize; 2] = [938472061, 958472071];

fn main() {
    input! {
        N: usize,
        A: [String; N],
    }

    let A = A
        .iter()
        .map(|x| BigUint::from_str(x).unwrap())
        .collect_vec();

    debug!(A);

    let mods = A
        .iter()
        .map(|a| {
            PRIMES
                .iter()
                .map(|&p| (a.clone() % p).to_usize().unwrap())
                .collect_vec()
        })
        .collect_vec();

    debug!(mods);

    let mut map = HashMap::new();

    for i in 0..N {
        for j in 0..N {
            let mul = mods[i]
                .iter()
                .zip(&mods[j])
                .zip(&PRIMES)
                .map(|((a, b), p)| a * b % p)
                .collect_vec();

            *map.entry(mul).or_insert(0) += 1;
        }
    }

    let mut ans = 0;

    for k in 0..N {
        if let Some(v) = map.get(&mods[k]) {
            ans += v;
        }
    }

    println!("{ans}");
}

const INF: usize = 1001001001001001001;

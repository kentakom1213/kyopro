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
        S: String,
        Q: usize,
        CD: [(char, char); Q]
    }

    // 位置を記憶
    let mut idx = HashMap::new();

    for (i, c) in S.chars().enumerate() {
        idx.entry(c).or_insert_with(Vec::new).push(i);
    }

    for &(c, d) in &CD {
        if let Some(mut cs) = idx.remove(&c) {
            if let Some(mut ds) = idx.remove(&d) {
                if cs.len() < ds.len() {
                    ds.append(&mut cs);
                    idx.insert(d, ds);
                } else {
                    cs.append(&mut ds);
                    idx.insert(d, cs);
                }
            } else {
                idx.insert(d, cs);
            }
        }
    }

    let mut chars = vec![];

    for (&c, cs) in idx.iter() {
        for &i in cs {
            chars.push((i, c));
        }
    }

    chars.sort();

    let ans = chars.iter().map(|(_, c)| *c).join("");

    println!("{ans}");

    // let mut map = HashMap::new();

    // for &(c, d) in &CD {
    //     map.insert(c, d);
    // }

    // // 置き換え
    // let mut ans = String::new();

    // for c in S.chars() {
    //     if let Some(&d) = map.get(&c) {
    //         ans.push(d);
    //     } else {
    //         ans.push(c);
    //     }
    // }

    // println!("{ans}");
}

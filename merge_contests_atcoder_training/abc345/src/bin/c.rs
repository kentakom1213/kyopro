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
        S: String,
    }

    // 同じ文字のペア
    let mut cnt = HashMap::new();

    for c in S.chars() {
        let x = cnt.entry(c).or_insert(0);
        *x += 1_usize;
    }

    let N = S.len();
    let mut ans = N * (N - 1) / 2;

    let mut same = false;
    for (_, &i) in &cnt {
        if i > 1 {
            ans -= i * (i - 1) / 2;
            same = true;
        }
    }

    if same {
        ans += 1;
    }

    debug!(cnt);

    println!("{ans}");
}

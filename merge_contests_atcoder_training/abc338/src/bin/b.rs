#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}, fastout};

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

/// `chmax!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最大のものを、`x1`に代入する
/// - 代入があったとき、`true`を返す
#[macro_export]
macro_rules! chmax {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
    ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
        chmax! {
            $a,
            ($b).max($c)
            $(,$other)*
        }
    }}
}

fn main() {
    input! {
        S: String
    }

    let mut map = BTreeMap::new();

    let mut max = 0;
    for c in S.chars() {
        let e = map.entry(c).or_insert(0);
        *e += 1;

        chmax! {
            max,
            *e
        };
    }

    for (k, v) in map {
        if v == max {
            println!("{k}");
            return;
        }
    }
}

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::cmp::Reverse;

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use rustc_hash::FxHashSet;

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
        N: usize,
        M: usize,
        L: usize,
        A: [usize; N],
        B: [usize; M],
        CD: [(Usize1, Usize1); L]
    }

    let NG = CD.iter().cloned().collect::<FxHashSet<_>>();

    debug!(NG);

    let B_sorted = B
        .iter()
        .cloned()
        .enumerate()
        .sorted_by_key(|&(i, v)| Reverse(v))
        .collect_vec();

    debug!(B_sorted);

    // Aを全探索
    let mut ans = 0;

    for (c, &a) in A.iter().enumerate() {
        // うしろから見ていく
        for &(d, b) in &B_sorted {
            if !NG.contains(&(c, d)) {
                chmax! {
                    ans,
                    a + b
                };
                break;
            }
        }
    }

    println!("{}", ans);
}

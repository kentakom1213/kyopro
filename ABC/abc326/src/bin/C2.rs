// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![warn(clippy::default_numeric_fallback)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Usize1}};
use superslice::Ext;

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

/// `a < b` のとき、`a`を`b`に置き換え、trueを返す
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
        mut A: [usize; N],
    }

    A.sort();

    // 決め打ち全探索
    let mut ans = 0;
    for i in 0..N {
        let r = A.lower_bound(&(A[i] + M));
        // println!("{:?}", &A[i..r]);
        chmax!(
            ans,
            r - i
        );
    }

    println!("{}", ans);
}

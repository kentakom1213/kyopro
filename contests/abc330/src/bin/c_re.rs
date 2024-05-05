// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::ops::Range;

// imports
use itertools::Itertools;
use num_integer::Roots;
use proconio::{input, marker::{Chars, Bytes, Usize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

/// `chmin!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最小のものを、`x1`に代入する
/// - 代入があったとき、`true`を返す
#[macro_export]
macro_rules! chmin {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
    ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
        chmin! {
            $a,
            ($b).min($c)
            $(,$other)*
        }
    }};
}

fn main() {
    input! {
        D: usize
    }

    let mut ans = INF;

    // xを 0~√D まで全探索
    let mut x = 0;
    while x * x <= D {
        let y = (D - x * x).sqrt();
        chmin! {
            ans,
            (x * x + y * y).abs_diff(D),
            (x * x + (y + 1) * (y + 1)).abs_diff(D),
        };

        x += 1;
    }

    println!("{}", ans);
}

const INF: usize = 1001001001001001001;

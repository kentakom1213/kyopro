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

use proconio::{input, marker::{Chars, Bytes, Usize1}};

fn main() {
    input! {
        N: usize,
        M: usize,
        S: [(isize, isize); N],
        C: [(isize, isize); M],
    }

    // 学生を全探索
    for &(a, b) in &S {
        let mut min_dist = isize::MAX;
        let mut idx = 0;
        for (i, &(x, y)) in C.iter().enumerate() {
            if chmin! {
                min_dist,
                (a - x).abs() + (b - y).abs()
            } {
                idx = i + 1;
            }
        }
        println!("{idx}");
    }
}

const INF: usize = 1001001001001001001;

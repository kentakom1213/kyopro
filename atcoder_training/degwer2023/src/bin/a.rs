// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

fn main() {
    input! {
        N: usize,
        K: usize,
        T: isize,
        mut A: [isize; K]
    }

    // ソート
    A.sort();

    // 必要になったら修正する
    let mut ans = 0_usize;
    let mut last = -INF;
    for &a in &A {
        if last + T > a {
            ans += 1;
        } else {
            last = a;
        }
    }

    println!("{}", ans);
}

const INF: isize = 1001001001001001001;

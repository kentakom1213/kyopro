// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

fn main() {
    input! {
        S: Chars,
        T: Chars
    }

    if S.len() != T.len() {
        println!("-1");
        return;
    }

    let N = S.len();

    // sのスタート位置
    for i in 0..N {
        if (0..N).all(|j| T[(i + j) % N] == S[j]) {
            println!("{}", i);
            return;
        }
    }

    println!("-1");
}

const INF: usize = 1001001001001001001;

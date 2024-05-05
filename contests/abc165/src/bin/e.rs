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

use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

fn main() {
    input! {
        N: usize,
        M: usize
    }

    let mut ans = vec![[0, 0]; M];

    // アドホックに割当て
    for i in 0..M / 2 {
        ans[i] = [i + 1, M - i];
    }

    debug2D!(ans);

    for i in 0..(M + 1) / 2 {
        ans[M - i - 1] = [M + i + 1, 2 * M - i + 1];
    }

    debug2D!(ans);

    for &[a, b] in &ans {
        println!("{a} {b}");
    }
}

const INF: usize = 1001001001001001001;

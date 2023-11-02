// https://atcoder.jp/contests/cf17-final/tasks/cf17_final_b

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// main
fn main() {
    input! {
        S: String,
    }

    let cnt = S.chars().fold([0, 0, 0], |[a, b, c], chr| match chr {
        'a' => {
            [a + 1, b, c]
        }
        'b' => {
            [a, b + 1, c]
        }
        _ => {
            [a, b, c + 1]
        }
    });

    debug!(cnt);

    let mini = *cnt.iter().min().unwrap();
    let maxi = *cnt.iter().max().unwrap();

    debug!(mini, maxi);

    println!("{}", ["NO", "YES"][(maxi - mini < 2) as usize]);
}

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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
        T: usize,
    }

    for _ in 0..T {
        solve();
    }
}

fn solve() {
    input! {
        N: usize,
        S: String,
    }

    // K, Pそれぞれのnim和
    let (nimK, nimP) = S
        .split('-')
        .map(|s| {
            let mut cnt = (0_usize, 0_usize);
            for c in s.chars() {
                if c == 'K' {
                    cnt.0 += 1;
                } else {
                    cnt.1 += 1;
                }
            }
            cnt
        })
        .fold((0, 0), |(k, p), (a, b)| (k ^ a, p ^ b));

    debug!(nimK, nimP);

    if nimK ^ nimP == 0 {
        println!("P");
    } else {
        println!("K");
    }
}

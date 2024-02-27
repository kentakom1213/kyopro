#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use itertools::Itertools;
use num_integer::{gcd, lcm};
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
        N: usize,
        M: usize,
        K: usize,
    }

    let LCM = lcm(N, M);

    // x未満の正整数のうち，ちょうどN,Mのどちらか一方でのみ割り切れるものの個数
    let cnt = |x: usize| -> usize { x / N + x / M - 2 * (x / LCM) };

    // Nの倍数で二分探索

    let mut ng = 0;
    let mut ok = INF;
    while ok - ng > 1 {
        let mid = (ng + ok) / 2;
        if cnt(mid) >= K {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    debug!(ok, ng);
    debug!(cnt(ok), cnt(ng));

    println!("{ok}");
}

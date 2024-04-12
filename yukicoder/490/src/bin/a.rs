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
    }

    // Nの正の約数の個数をカウント
    let mut cnt = 0;

    let mut i = 1;
    while i * i <= N {
        if i * i == N {
            cnt += 1;
        } else if N % i == 0 {
            cnt += 2;
        }
        i += 1;
    }

    // 偶奇で場合分け
    if cnt % 2 == 0 {
        println!("K");
    } else {
        println!("P");
    }
}

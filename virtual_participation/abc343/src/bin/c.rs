#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}, fastout};
use superslice::Ext;

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
        N: usize
    }

    // 回文立方数を列挙
    let tet = {
        let mut tet = vec![];
        let mut i = 0;
        while i * i * i <= N {
            // 回文判定
            let t = i * i * i;

            let s = t.to_string();
            if s == s.chars().rev().collect::<String>() {
                tet.push(t);
            }

            i += 1;
        }
        tet
    };

    let ans = tet.upper_bound(&N) - 1;

    println!("{}", tet[ans]);
}

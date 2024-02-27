#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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
        H: usize,
        W: usize,
        N: usize,
        T: String,
        S: [Chars; H]
    }

    // スタート地点になりうるか
    let can_move = |r: usize, c: usize| -> bool {
        if S[r][c] == '#' {
            return false;
        }

        let mut cr = r;
        let mut cc = c;
        for c in T.chars() {
            match c {
                'L' => {
                    cc -= 1;
                }
                'R' => {
                    cc += 1;
                }
                'U' => {
                    cr -= 1;
                }
                'D' => {
                    cr += 1;
                }
                _ => (),
            };

            if S[cr][cc] == '#' {
                return false;
            }
        }
        true
    };

    let mut ans = 0;

    for r in 0..H {
        for c in 0..W {
            if can_move(r, c) {
                ans += 1;
            }
        }
    }

    println!("{ans}");
}

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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: String,
    }

    let mut cnt = [0; 3];

    for c in N.chars() {
        let d = c.to_digit(10).unwrap() as usize;
        cnt[d % 3] += 1;
    }

    let mut ans = 20;
    let mut rem = cnt[1] + cnt[2] * 2;

    if rem % 3 == 0 {
        ans = 0;
    }

    // 何桁消すか
    for a in 0..=cnt[0] {
        for b in 0..=cnt[1] {
            for c in 0..=cnt[2] {
                if (rem - (b + 2 * c)) % 3 == 0 {
                    ans = ans.min(a + b + c);
                }
            }
        }
    }

    if ans >= N.len() {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

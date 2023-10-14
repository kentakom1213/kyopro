// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![warn(clippy::default_numeric_fallback)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Usize1}};

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
        N: usize,
        mut S: Chars,
    }

    S.sort();

    // 平方数を列挙
    const MAX: usize = 5_000_000;
    let mut ans = 0_usize;

    for i in 0..MAX {
        let s = (i * i).to_string();
        let mut tmp = vec!['0'; N.saturating_sub(s.len())];
        for c in s.chars().sorted() {
            tmp.push(c);
        }
        if &S == &tmp {
            ans += 1;
        }
    }

    println!("{}", ans);
}

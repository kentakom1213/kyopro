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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        A: [Usize1; N]
    }

    let counter = A.iter().fold(vec![0_usize; N], |mut cnt, &a| {
        cnt[a] += 1;
        cnt
    });

    let all = counter.iter().fold(0, |acc, i| acc + i * i.saturating_sub(1) / 2);

    let ans = A.iter()
        .map(|&a| (all + 1).saturating_sub(counter[a]))
        .join("\n");

    println!("{}", ans);
}

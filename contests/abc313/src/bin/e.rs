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

fn main() {
    input! {
        N: usize,
        S: String
    }
    let mut ans = 0;
    let mut last = '1';

    for c in S[1..].chars().rev() {
        if last > '1' && c > '1' {
            println!("-1");
            return;
        }
        ans += 1;
        ans += ans * (c as usize - '1' as usize);
        ans %= MOD;
        last = c;
    }

    println!("{ans}");
}

const MOD: usize = 998244353;

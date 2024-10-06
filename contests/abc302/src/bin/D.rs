// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};
use superslice::Ext;

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
        N: usize,
        M: usize,
        D: isize,
        mut A: [isize; N],
        mut B: [isize; M],
    }

    A.sort();
    B.sort();

    let mut ans = -1;

    // Aを全探索
    for &a in &A {
        let mini = a.saturating_sub(D);
        let maxi = a + D + 1;
        let left = B.lower_bound(&mini);
        let right = B.lower_bound(&maxi);
        let rng = &B[left..right];
        debug!(rng);
        if rng.len() >= 1 {
            let tmp = a + rng.last().unwrap();
            ans = ans.max(tmp);
        }
    }

    println!("{}", ans);
}

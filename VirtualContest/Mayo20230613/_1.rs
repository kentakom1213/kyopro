// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

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
type Graph = Vec<Vec<usize>>;

// main
fn main() {
    input! {
        N: usize,
        K: usize,
        Q: usize,
        A: [Usize1; Q],
    }

    let mut cnt = vec![0_usize; N];
    for &a in &A {
        cnt[a] += 1;
    }

    debug!(&cnt);

    for i in 0..N {
        if (K + cnt[i]).saturating_sub(Q) == 0 {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}

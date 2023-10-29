// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// main
fn main() {
    input! {
        N: usize,
        mut T: usize,
        C: [usize; N],
        R: [usize; N],
    }

    if !C.iter().any(|&v| v == T) {
        T = C[0];
    }

    let mut res = (0, 0);

    for (i, (&c, &r)) in C.iter().zip(R.iter()).enumerate() {
        if c == T {
            if res.0 < r {
                res = (r, i + 1);
            }
        }
    }

    println!("{}", res.1);
}

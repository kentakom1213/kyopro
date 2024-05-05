//              D - Unbalanced             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc043/tasks/arc059_b
// ----------------------------------------

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
        S: Chars
    }

    let N = S.len();

    // ααを含むか
    for i in 1..N {
        if S[i - 1] == S[i] {
            println!("{} {}", i, i + 1);
            return;
        }
    }

    // αβαを含むか
    for i in 2..N {
        if S[i - 2] == S[i] {
            println!("{} {}", i - 1, i + 1);
            return;
        }
    }

    println!("-1 -1");
}

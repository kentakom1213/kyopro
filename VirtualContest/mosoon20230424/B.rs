// https://atcoder.jp/contests/abc013/tasks/abc013_1

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// main
fn main() {
    input! {
        X: char,
    }

    for (i, c) in "ABCDE".chars().enumerate() {
        if c == X {
            println!("{}", i + 1);
            return;
        }
    }
}
// https://atcoder.jp/contests/abc137/tasks/abc137_d

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use std::{cmp::Reverse, collections::VecDeque};
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

fn debug_2d<T: std::fmt::Debug>(array: &Vec<Vec<T>>) {
    #[cfg(debug_assertions)]
    for row in array {
        println!("{:?}", row);
    }
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// ## 方針
/// - 貪欲に解く
fn main() {
    input! {
        N: usize,
        M: usize,
        mut AB: [(usize, usize); N],
    }

    // 期日が遠いものから順番に、報酬が多い順に取っていく
    AB.sort_by_key(|&(a, b)| (Reverse(b), a));

    debug!(&AB);

    let mut ans = 0;
    let mut ptr = 0;

    for i in (0..M).rev() {
        while ptr < AB.len() && AB[ptr].0 > i {
            ptr += 1;
        }
        if ptr >= AB.len() { break; }
        ans += AB[ptr].1;
        ptr += 1;
    }

    println!("{}", ans);
}
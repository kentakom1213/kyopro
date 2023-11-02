// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}, fastout};

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

/// ## ord
/// `A`を`0`とするascii文字(`A~Za~z`)のインデックスを返す
pub fn ord(c: char) -> usize {
    let a = 'A' as u32;
    let c = c as u32;
    (c - a) as usize
}
/// ## chr
/// `chr(0) = A`であるようなascii文字(`A~Za~z`)を返す
pub fn chr(i: usize) -> char {
    let a = 'A' as u32;
    char::from_u32(a + i as u32).unwrap()
}

#[fastout]
fn main() {
    input! {
        S: String,
        Q: usize,
        queries: [(usize, Usize1); Q]
    }

    let Sord: Vec<usize> = S.chars().map(ord).collect();

    for &(k, idx) in &queries {
        println!("{}", f(k, idx, &Sord));
    }
}

fn g(x: usize, add: usize) -> char {
    chr((x + add) % 3)
}

fn f(k: usize, idx: usize, S: &[usize]) -> char {
    if k == 0 {
        chr(S[idx])
    } else if idx == 0 {
        g(S[idx], k)
    } else {
        g(ord(f(k - 1, idx >> 1, S)), idx % 2 + 1)
    }
}

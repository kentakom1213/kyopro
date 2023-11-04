// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}};

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
        A: [[Usize1; 9]; 9]
    }

    let mut is_row = true;
    
    for r in 0..9 {
        let mut tmp = vec![false; 9];
        for c in 0..9 {
            tmp[A[r][c]] = true;
        }
        is_row &= tmp.iter().all(|x| *x);
    }

    let mut is_col = true;
    
    for c in 0..9 {
        let mut tmp = vec![false; 9];
        for r in 0..9 {
            tmp[A[r][c]] = true;
        }
        is_col &= tmp.iter().all(|x| *x);
    }

    let mut is_block = true;

    for br in 0..3 {
        for bc in 0..3 {
            let mut tmp = vec![false; 9];
            for r in 0..3 {
                for c in 0..3 {
                    tmp[A[br * 3 + r][bc * 3 + c]] = true;
                }
            }
            is_block &= tmp.iter().all(|x| *x);
        }
    }

    if is_row && is_col && is_block {
        println!("Yes");
    } else {
        println!("No");
    }
}

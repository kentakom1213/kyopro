// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::VecDeque;

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
        X: usize,
        Y: usize,
        A: [usize; N],
    }

    // 条件を満たす可能性がある区間のみ
    let mut B = vec![];

    for &a in &A {
        if Y <= a && a <= X {
            B.push(a);
        } else {
            debug!(B);
            B.clear();
        }
    }
    debug!(B);
}

/// 条件を満たす区間の個数をカウント
fn shakutori(B: &mut Vec<usize>) -> usize {
    
}

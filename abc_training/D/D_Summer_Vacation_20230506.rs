//           D - Summer Vacation
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc137/tasks/abc137_d
// ----------------------------------------

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
use std::collections::BinaryHeap;

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

// main
fn main() {
    input! {
        N: usize,
        M: usize,
        mut AB: [(usize, usize); N],
    }

    // ソート
    AB.sort_by_key(|&(a, b)| a);

    let mut que = BinaryHeap::new();
    let mut ptr = 0; // ABの何個目まで見たか
    let mut ans = 0;

    for rem in 1..=M {
        while ptr < N && AB[ptr].0 <= rem {
            let (a, b) = AB[ptr];
            que.push((b, a));
            ptr += 1;
        }
        debug!(&que);
        if let Some((a, b)) = que.pop() {
            ans += a;
        }
    }

    println!("{}", ans);
}

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};
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

fn main() {
    input! {
        N: usize,
        A: [Usize1; N],
        Q: usize,
        LRX: [(Usize1, usize, Usize1); Q]
    }

    let mut idx = vec![vec![]; N];

    for (i, &c) in A.iter().enumerate() {
        idx[c].push(i);
    }

    let mut ans = vec![];

    for &(l, r, x) in &LRX {
        let a = idx[x].lower_bound(&l);
        let b = idx[x].lower_bound(&r);
        let res = b - a;
        ans.push(res);
    }

    println!("{}", ans.iter().join("\n"));
}

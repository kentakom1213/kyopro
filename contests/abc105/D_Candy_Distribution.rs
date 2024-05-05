//          D - Candy Distribution
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc105/tasks/abc105_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::BTreeMap;

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

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

// main
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize; N],
    }

    // MODを取る累積和
    let mut S = vec![0; N];
    S[0] = A[0] % M;

    for i in 1..N {
        S[i] = (S[i - 1] + A[i]) % M;
    }

    debug!(&S);

    // それぞれの個数をカウント
    let mut ans = 0_usize;
    let mut counter = BTreeMap::new();

    for &v in &S {
        let cur = counter.entry(v).or_insert(0);

        if v == 0 {
            ans += 1;
        }

        ans += *cur;
        *cur += 1;
    }

    debug!(&counter);

    println!("{}", ans);
}

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![warn(clippy::default_numeric_fallback)]

use std::cmp::Reverse;

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
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
        mut A: [usize; M],
        mut SS: [Chars; N],
    }

    // 点数でソート
    for s in SS.iter_mut() {
        *s = s
            .iter()
            .enumerate()
            .sorted_by_key(|(i, _)| Reverse(A[*i]))
            .map(|(i, c)| *c)
            .collect();
    }

    A.sort();
    A.reverse();

    debug!(&SS);
    debug!(&A);

    // 各選手の点数を計算
    let mut P = vec![0; N];
    for (n, s) in SS.iter().enumerate() {
        let mut cnt = 0;
        for (i, &c) in s.iter().enumerate() {
            if c == 'o' {
                cnt += A[i];
            }
        }
        P[n] = cnt + n + 1;
    }

    debug!(&P);

    // 各選手について計算
    for (i, s) in SS.iter().enumerate() {
        // その選手を除いたときのスコアの最大値
        let maxi = *P[..i].iter().chain(P[i + 1..].iter()).max().unwrap();

        // 貪欲にとる
        let mut cnt = 0_usize;
        let mut cur = P[i];
        for j in 0..M {
            if cur > maxi {
                break;
            }
            // 得点
            if s[j] == 'x' {
                cnt += 1;
                cur += A[j];
            }
        }

        println!("{}", cnt);
    }
}

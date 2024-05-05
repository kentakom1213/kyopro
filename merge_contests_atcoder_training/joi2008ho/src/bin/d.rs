// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::BTreeSet;

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
        N: usize,
        M: usize,
        mut P: [usize; N],
    }

    P.push(0);

    // 2つ組を列挙
    let mut combi = BTreeSet::new();

    for i in 0..=N {
        for j in 0..=N {
            combi.insert(P[i] + P[j]);
        }
    }

    // 全探索
    let mut ans = 0;

    for &ab in &combi {
        if ab > M {
            break;
        }
        if let Some(&cd) = combi.range(..=M.saturating_sub(ab)).next_back() {
            if ans < ab + cd && ab + cd <= M {
                ans = ab + cd;
            }
        }
    }

    println!("{}", ans);
}

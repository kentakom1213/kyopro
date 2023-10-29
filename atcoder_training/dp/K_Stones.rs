//                K - Stones
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/dp/tasks/dp_k
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
/// - ミニマックス法？
/// ## 解説
/// - dp(i, j) := 石の個数がi個のときに、直後の手番の人が勝てるか
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N],
    }

    // dp
    let mut dp = vec![false; K + 1];

    for i in 1..=K {
        for j in 0..N {
            dp[i] |= i >= A[j] && dp[i - A[j]] == false;
        }
    }

    if dp[K] {
        println!("First");
    } else {
        println!("Second");
    }
}

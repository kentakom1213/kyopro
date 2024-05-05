//               C - chokudai
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc211/tasks/abc211_c
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// constant
const MOD: usize = 1_000_000_007;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();
const CHOKUDAI: &str = "chokudai";

macro_rules! set_madd {
    ( $a:expr, $($rem:expr),+ $(,)* ) => {{
        let sum = madd!( $($rem),+ );
        $a = ($a + sum) % MOD;
    }};
}

macro_rules! madd {
    ( $a:expr ) => {{
        $a % MOD
    }};
    ( $a:expr, $b:expr ) => {{
        ($a + $b) % MOD
    }};
    ( $a:expr, $($rem:expr),+ $(,)* ) => {{
        ($a + madd!( $($rem),+) ) % MOD
    }};
}

// solve
fn main() {
    input! {
        S: Chars,
    }
    let C: Vec<char> = CHOKUDAI.chars().collect();

    let mut dp = vec![vec![0; 9]; S.len() + 1];
    for i in 0..=S.len() {
        dp[i][0] = 1;
    }

    // dp更新
    for i in 1..=S.len() {
        for j in 1..=8 {
            if S[i-1] == C[j-1] {
                set_madd!(
                    dp[i][j],
                    dp[i-1][j],
                    dp[i-1][j-1],
                );
            } else {
                dp[i][j] = dp[i-1][j];
            }
        }
    }

    println!("{}", dp[S.len()][8]);
}

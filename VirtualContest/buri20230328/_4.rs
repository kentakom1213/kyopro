// https://atcoder.jp/contests/abc266/tasks/abc266_d

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: isize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// `a < b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmax {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

// solve
fn main() {
    input! {
        N: usize,
        snukes: [(usize, usize, isize); N],
    }

    // dp
    let mut field = vec![vec![0; 5]; 101010];
    let mut dp = vec![vec![-INF; 5]; 101010];
    dp[0][0] = 0;

    for &(t, x, a) in &snukes {
        field[t][x] += a;
    }

    for t in 0..=100000 {
        for x in 0..5 {
            if dp[t][x] == INF {
                continue;
            }

            // 左に移動
            if x > 0 {
                chmax!(
                    dp[t+1][x-1],
                    dp[t][x] + field[t][x],
                );
            }
            // そのまま
            chmax!(
                dp[t+1][x],
                dp[t][x] + field[t][x],
            );
            // 右に移動
            if x < 4 {
                chmax!(
                    dp[t+1][x+1],
                    dp[t][x] + field[t][x],
                );
            }
        }
    }

    println!("{}", dp[100001].iter().max().unwrap());
}
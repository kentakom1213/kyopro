//            D - Step Up Robot            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc289/tasks/abc289_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// solve
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        M: usize,
        B: [usize; M],
        X: usize,
    }

    let mut dp = vec![0; X+1];
    dp[0] = 1;

    for &b in &B {
        dp[b] = -1;
    }

    // dp更新
    for i in 0..X {
        for &a in &A {
            if dp[i] != 1 { continue; }
            if i + a <= X && dp[i+a] == 0 {
                dp[i+a] = 1;
            }
        }
    }

    if dp[X] == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}

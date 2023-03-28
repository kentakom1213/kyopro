//               D - Marking               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc290/tasks/abc290_d
// ----------------------------------------

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
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// main
fn main() {
    input! {
        T: usize,
    }
    for _ in 0..T {
        solve();
    }
}

/// 1. gcd(N, D) == 1
/// → K * D % N
/// 2. gcd(N, D) != 1
/// → K % N + N / D * D
fn solve() {
    input! {
        N: usize,
        D: usize,
        K: Usize1,
    }
    let L = N / gcd(N, D);
    let ans = K / L + K * (D%N) % N;

    println!("{}", ans);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

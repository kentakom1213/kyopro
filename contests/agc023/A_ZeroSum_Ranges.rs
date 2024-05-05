//           A - Zero-Sum Ranges           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/agc023/tasks/agc023_a
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
        A: [isize; N],
    }

    // 累積和をとる
    let mut S = vec![0; N + 1];
    for i in 0..N {
        S[i + 1] = S[i] + A[i];
    }

    // それぞれの個数をカウント
    let mut cnt = BTreeMap::new();
    for &v in &S {
        *cnt.entry(v).or_insert(0) += 1;
    }

    // 組合せを数え上げる
    let mut ans = 0_usize;
    for (_, &v) in &cnt {
        ans += v * (v - 1) / 2;
    }

    println!("{}", ans);
}
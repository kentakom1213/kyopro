//          E - Make it Palindrome
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc290/tasks/abc290_e
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
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// ## 方針
/// - グラフとしてみる
fn main() {
    input! {
        N: usize,
        
        A: [usize; N],
    }

    // all_pairs: 全てのペア
    let all_pairs = (1..=N).map(|i| (N - i + 1) * i / 2).sum::<usize>();

    println!("{}", all_pairs);
}

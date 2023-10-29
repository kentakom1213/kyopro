//           D -  Three Days Ago           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc295/tasks/abc295_d
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
        S: String,
    }

    // 個数 mod 2をカウント
    let mut state = 0;
    let mut cnt: Vec<usize> = vec![0; 1 << 10];
    cnt[0] += 1;
    let mut ans = 0;

    for (i, c) in S.chars().enumerate() {
        let d = c.to_digit(10).unwrap() as usize;
        state ^= 1 << d;
        ans += cnt[state];
        cnt[state] += 1;
    }

    println!("{}", ans);
}
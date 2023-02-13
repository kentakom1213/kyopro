//               B - 高橋くんの集計               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc015/tasks/abc015_2
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
    }

    let mut cnt_n = 0;
    let mut cnt_bug = 0;
    for &a in &A {
        if a > 0 {
            cnt_n += 1;
            cnt_bug += a;
        }
    }

    // 切り上げ
    let ans = (cnt_bug + cnt_n - 1) / cnt_n;
    println!("{}", ans);
}

//            D - Non-decreasing           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc081/tasks/arc086_b
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
const INF: isize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// solve
fn main() {
    input! {
        N: usize,
        A: [isize; N],
    }

    let mut ans = vec![];
    
    // 最小値,最大値とそのインデックスを取得
    let (mut max, mut max_idx) = (-INF, 0);
    let (mut min, mut min_idx) = (INF, 0);
    for (i, &v) in A.iter().enumerate() {
        if v > max {
            max = v;
            max_idx = i;
        }
        if v < min {
            min = v;
            min_idx = i;
        }
    }

    // |max| >= |min| -> 正に揃える
    if max.abs() >= min.abs() {
        // 正に揃える
        for i in 0..N {
            ans.push((max_idx, i));
        }
        // 前方から累積和をとる
        for i in 0..N-1 {
            ans.push((i, i+1));
        }
    }
    // |min| < |max| -> 負に揃える
    else {
        // 負に揃える
        for i in 0..N {
            ans.push((min_idx, i));
        }
        // 後方から累積和をとる
        for i in (0..N-1).rev() {
            ans.push((i+1, i));
        }
    }

    // 表示
    println!("{}", ans.len());
    ans.iter().for_each(|(x, y)| println!("{} {}", x+1, y+1));
}
//                  B - 回転                 
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc004/tasks/abc004_2
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
// https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use proconio::input;

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// solve
fn main() {
    input! {
        S: [[char; 4]; 4],
    }

    for i in (0..4).rev() {
        for j in (0..4).rev() {
            print!("{} ", S[i][j]);
        }
        println!();
    }
}

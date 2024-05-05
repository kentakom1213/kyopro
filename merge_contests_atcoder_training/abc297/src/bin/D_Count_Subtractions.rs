//          D - Count Subtractions         
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc297/tasks/abc297_d
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

/// ## 方針
/// ユークリッドの互除法を用いて実装する
/// - 計算量：O(log(max(A, B)))
fn main() {
    input! {
        A: usize, B: usize
    }

    let mut ans = 0;
    let mut a = A;
    let mut b = B;

    while a > 0 && b > 0 {
        if a > b {
            ans += a / b;
            a %= b;
        } else {
            ans += b / a;
            b %= a;
        }
    }

    println!("{}", ans - 1);
}

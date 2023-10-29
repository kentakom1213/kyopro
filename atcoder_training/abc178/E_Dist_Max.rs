//               E - Dist Max
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc178/tasks/abc178_e
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
/// - マンハッタン距離は45度回転
fn main() {
    input! {
        N: usize,
        P: [(isize, isize); N],
    }

    // 45度回転
    let P45: Vec<(isize, isize)> = P.iter().map(|&(x, y)| (x - y, x + y)).collect();

    // 左右で比較する
    let left = P45.iter().min_by_key(|(x, y)| *x).unwrap().0;
    let right = P45.iter().max_by_key(|(x, y)| *x).unwrap().0;

    // 上下で比較する
    let bottom = P45.iter().min_by_key(|(x, y)| *y).unwrap().1;
    let top = P45.iter().max_by_key(|(x, y)| *y).unwrap().1;

    let ans = (right - left).max(top - bottom);
    println!("{}", ans);
}

//               B - Magic 3
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc190/tasks/abc190_b
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

// solve
fn main() {
    input! {
        N: usize,
        S: usize,
        D: usize,
        XY: [(usize, usize); N],
    }

    let can_damage = XY.iter().any(|&(x, y)| x < S && y > D);

    if can_damage {
        println!("Yes");
    } else {
        println!("No");
    }
}

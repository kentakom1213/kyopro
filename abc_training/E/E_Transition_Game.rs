//           E - Transition Game
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc296/tasks/abc296_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
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

// main
fn main() {
    input! {
        N: usize,
        A: [Usize1; N],
    }

    // ダブリング
    let mut double = vec![vec![0; N]; 30];
    for i in 0..N {
        double[0][i] = A[i];
    }
    for i in 1..30 {
        for j in 0..N {
            double[i][j] = double[i - 1][double[i - 1][j]];
        }
    }

    
}

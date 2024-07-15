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
        M: usize,
        K: usize,
        AB: [(usize, usize); M],
    }

    let mut ans = 0;
    let mut snow = 0;

    // ペアについて考える
    for i in 0..M - 1 {
        let span = AB[i + 1].0 - AB[i].0; // 次の降雪までの日数
        snow += AB[i].1; // 現在の積雪

        // 残りの雪がK未満になるまでの日数
        let rem = snow - K + 1;
    }
}

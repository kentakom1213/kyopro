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

// https://atcoder.jp/contests/abc107/tasks/arc101_a
fn main() {
    input! {
        N: usize,
        K: usize,
        X: [isize; N],
    }

    let mut ans = INF;
    for i in 0..=N-K {
        let (l, r) = (X[i], X[i+K-1]);
        let tmp = (r - l) + min(l.abs(), r.abs());
        if ans > tmp {
            ans = tmp;
        }
    }

    println!("{}", ans);
}
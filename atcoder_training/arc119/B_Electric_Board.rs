//            B - Electric Board           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc119/tasks/arc119_b
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
        S: String,
        T: String,
    }

    let mut s0 = vec![];
    let mut t0 = vec![];
    for (i, (s, t)) in S.chars().zip(T.chars()).enumerate() {
        if s == '0' {
            s0.push(i);
        }
        if t == '0' {
            t0.push(i);
        }
    }

    if s0.len() != t0.len() {
        println!("-1");
    }
    else {
        let mut diff = 0;
        for (i, j) in s0.iter().zip(t0.iter()) {
            if i != j {
                diff += 1;
            }
        }
        println!("{}", diff);
    }
}

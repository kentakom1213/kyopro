//              C - 高橋くんのバグ探し              
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc015/tasks/abc015_3
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
        K: usize,
        T: [[usize; K]; N],
    }

    // 5^5 = 3125通りを試す
    let has_bug = choices(N, 0, 0, &T);

    if has_bug {
        println!("Found");
    } else {
        println!("Nothing");
    }
}

fn choices(N: usize, i: usize, val: usize, T: &Vec<Vec<usize>>) -> bool {
    if i == N {
        return val == 0;
    }
    let mut res = false;
    for &v in &T[i] {
        res |= choices(N, i+1, val ^ v, T);
    }
    res
}

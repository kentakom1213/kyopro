//          B - Call the ID Number         
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc293/tasks/abc293_b
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
use itertools::Itertools;
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
        mut A: [Usize1; N],
    }

    for i in 0..N {
        let nxt = A[i];
        if nxt < N {
            A[nxt] = INF;
        }
    }

    let ans: Vec<usize> = A.iter().enumerate().filter(|(i, &v)| v < INF).map(|t| t.0 + 1).collect();
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
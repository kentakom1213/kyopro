//                 D - Bank                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc294/tasks/abc294_d
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

/// ## 
fn main() {
    input! {
        N: usize,
        Q: usize,
    }

    let mut top = 1;
    let mut called = BTreeSet::new();

    for _ in 0..Q {
        input! {q: usize}
        match q {
            1 => {
                called.insert(top);
                top += 1;
            },
            2 => {
                input! {x: usize}
                called.remove(&x);
            },
            3 => {
                let res = called.iter().next().unwrap();
                println!("{}", res);
            },
            _ => unreachable!(),
        }
    }
}
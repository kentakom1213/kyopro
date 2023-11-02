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
        a: usize,
        b: usize,
        x: usize,
        y: usize,
    }

    if a == b {
        println!("{}", x);
    } else if a < b {
        let u = x + (b - a) * y;  // 階段 + 廊下
        let v = ((b - a) * 2 + 1) * x;  // 廊下のみ
        println!("{}", u.min(v));
    } else {
        let u = x + (a - b - 1) * y;  // 階段 + 廊下
        let v = ((a - b) * 2 - 1) * x;  // 廊下のみ
        println!("{}", u.min(v));
    }
}
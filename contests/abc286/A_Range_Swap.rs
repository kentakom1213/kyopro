//              A - Range Swap             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc286/tasks/abc286_a
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
        (N, P, Q, R, S): (usize, Usize1, usize, Usize1, usize),
        A: [usize; N],
    }

    for &v in &A[..P] {
        print!("{} ", v);
    }
    for &v in &A[R..S] {
        print!("{} ", v);
    }
    for &v in &A[Q..R] {
        print!("{} ", v);
    }
    for &v in &A[P..Q] {
        print!("{} ", v)
    }
    for &v in &A[S..] {
        print!("{} ", v);
    }
    println!();
}

//              C - 高橋くんと魔法の箱
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc019/tasks/abc019_3
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
        A: [usize; N],
    }

    // Aの全ての要素について、2で割り切れる限り割って、setに入れる
    let set = A
        .iter()
        .map(|&a| {
            let mut v = a;
            while v % 2 == 0 {
                v /= 2;
            }
            v
        })
        .collect::<BTreeSet<usize>>();

    println!("{}", set.len());
}

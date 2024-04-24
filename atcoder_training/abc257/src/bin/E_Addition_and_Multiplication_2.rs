//    E - Addition and Multiplication 2    
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc257/tasks/abc257_e
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

/// ## 方針
/// - 桁数を最大化 → 大きい数字を貪欲にとる
fn main() {
    input! {
        N: usize,
        C: [usize; 9],
    }

    // 桁数を最大化できるような組合せを求める
    // 桁数の最大値：N / min(C)
    let minC = *C.iter().min().unwrap();
    // (桁数, あまり)
    let d = N / minC;
    let mut rem = N % minC;

    // 貪欲に数字を取っていく
    let mut ans = vec![];
    for _ in 0..d {
        for (i, &c) in (1..=9).rev().zip(C.iter().rev()) {
            if c - minC <= rem {
                ans.push(i);
                rem -= c - minC;
                break;
            }
            if c == minC {
                ans.push(i);
                break;
            }
        }
    }

    for &v in &ans {
        print!("{}", v);
    }
    println!();
}
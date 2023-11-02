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

/// ## 問題
/// https://atcoder.jp/contests/abc189/tasks/abc189_e
/// ## 方針
/// - あらかじめ行列演算として表しておく
/// - i回目の操作の結果を行列の積として保持
/// - (Ai, Bi) @ [[x, y], [z, w]] -> 答え
fn main() {
    input! {
        N: usize,
        XY: [(isize, isize); N],
        M: usize,
    }
    let mut OP = vec![];
    for _ in 0..M {
        input!{t: usize}
        if t <= 2 {
            OP.push((t, 0));
        } else {
            input!{p: isize}
            OP.push((t, p));
        }
    }
    input! {
        Q: usize,
        AB: [(isize, isize); Q],
    }

    // solve

}
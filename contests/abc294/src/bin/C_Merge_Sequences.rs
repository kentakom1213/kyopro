//           C - Merge Sequences
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc294/tasks/abc294_c
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
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

/// ## 方針
/// - O(N＋M)で解く
/// - マージしながら答えを保存
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize; N],
        B: [usize; M],
    }

    // マージしながら答えを保存
    let (mut i, mut j) = (0, 0);
    let mut ans_a = vec![];
    let mut ans_b = vec![];
    let mut cur = 1;
    while i < N || j < M {
        if i < N && j < M {
            if A[i] < B[j] {
                ans_a.push(cur);
                i += 1;
            }
            else {
                ans_b.push(cur);
                j += 1;
            }
        }
        else if i < N {
            ans_a.push(cur);
            i += 1;
        }
        else {
            ans_b.push(cur);
            j += 1;
        }
        cur += 1;
    }

    // 出力
    println!("{}", ans_a.iter().join(" "));
    println!("{}", ans_b.iter().join(" "));
}

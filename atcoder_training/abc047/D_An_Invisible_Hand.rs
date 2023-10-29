//          D - An Invisible Hand          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc047/tasks/arc063_b
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// ## 問題
/// - 結構難しい
/// ### 考え方
/// - 一つの町で売買するりんごの個数に制限はないので、
/// もっともりんごが安い町でりんごを買い、最も高い町で売れば良い。
/// - ここで、0<=i<j<Nとなる整数の組(i, j)について、`A[j]-A[i]`が最大となるようなi, jのペアの個数を求める。
/// - 答えはこのペアの個数となる。
fn main() {
    input! {
        N: usize,
        T: usize,
        A: [usize; N],
    }

    // B[x] := A[j] - A[i] (i < j) の最大値を求める
    let mut min = A[0];
    let mut Bmax = 0;
    for &v in &A[1..] {
        if min < v && Bmax < v - min {
            Bmax = v - min;
        }
        if min > v {
            min = v;
        }
    }

    // Bmaxを満たすようなペアの個数を求める
    let mut ans = 0;
    let mut min = INF;
    for &v in &A {
        if min < v && Bmax == v - min {
            ans += 1;
        }
        if min > v {
            min = v;
        }
    }

    println!("{}", ans);
}
// F - Regular Triangle Inside a Rectangle 
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc292/tasks/abc292_f
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
use std::f64::consts::PI;

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// main
fn main() {
    input! {
        mut A: f64,
        mut B: f64,
    }
    // A < B になるように入れ替え
    if A > B {
        std::mem::swap(&mut A, &mut B);
    }
    
    let PI_3 = PI / 3.0;  // 60°
    let PI_6 = PI / 6.0;  // 30°
    let eps = 1e-10;

    // 2分探索
    let (mut ok, mut ng) = (0.0, PI_6);
    while (ok - ng).abs() > eps {
        let m = (ok + ng) / 2.0;
        let theta = m + PI_3;
        if A / m.cos() * theta.sin() <= B {
            ok = m;
        } else {
            ng = m;
        }
    }

    let ans = A / ok.cos();
    println!("{:.15}", ans);
}
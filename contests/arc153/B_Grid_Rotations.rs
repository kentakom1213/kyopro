//            B - Grid Rotations           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc153/tasks/arc153_b
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
/// - 行・列についての独立した問題として解く
/// - 行・列それぞれの反転は巡回シフトと考えていい
fn main() {
    input! {
        H: usize,
        W: usize,
        A: [Chars; H],
        Q: usize,
        AB: [(usize, usize); Q],
    }

    // 行・列に関して先頭の2要素がどこに行ったかを記録していく
    let mut row = (0, 1);
    let mut col = (0, 1);

    for &(a, b) in &AB {
        // 行について処理
        

        // 列について処理

    }
}

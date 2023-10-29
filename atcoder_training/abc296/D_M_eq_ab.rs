//                D - M<=ab                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc296/tasks/abc296_d
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

pub trait UsizeTools {
    fn abs_diff(&self, other: Self) -> Self;
    fn sqrt(&self) -> Self;
}

impl UsizeTools for usize {
    fn abs_diff(&self, other: Self) -> Self {
        if *self > other {
            *self - other
        } else {
            other - *self
        }
    }

    /// ## sqrt
    /// x^2がNを超えない最大のxを求める
    /// - 計算量：O(log(N))
    fn sqrt(&self) -> Self {
        let (mut ok, mut ng) = (0_usize, 1001001001001001001);
        while (ng - ok) > 1 {
            let m = (ok + ng) / 2;
            if m.saturating_mul(m) <= *self {
                ok = m;
            } else {
                ng = m;
            }
        }
        ok
    }
}

/// ## 方針
/// - 1 <= a <= b <= √M となるように全探索
fn main() {
    input! {
        N: usize,
        M: usize,
    }

    let mut ans = INF;

    for b in 1..=N.min(M.sqrt() + 10) {
        let a = (M + b - 1) / b;
        if a <= N {
            ans = ans.min(a * b);
        }
    }

    if ans == INF {
        println!("{}", -1);
    }
    else {
        println!("{}", ans);
    }
}
//                 D - XXOR                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc117/tasks/abc117_d
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
/// - 2進数で表し、桁ごとに考える
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N],
    }

    let mut x = 0;
    let mut ans = 0;

    // 上の桁から順に確定させていく
    for i in (0..60).rev() {
        let mut cnt = [0, 0];
        for &v in &A {
            cnt[(v >> i) & 1] += 1;
        }
        if cnt[0] > cnt[1] && x | (1_usize << i) <= K {
            ans += cnt[0] * (1_usize << i);
            x |= 1_usize << i;
        }
        else {
            ans += cnt[1] * (1_usize << i);
        }
    }

    println!("{}", ans);
}
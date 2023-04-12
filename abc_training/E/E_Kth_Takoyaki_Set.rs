//           E - Kth Takoyaki Set          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc297/tasks/abc297_e
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

/// ## 解説
/// - ダイクストラ法
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N]
    }

    let mut que = BinaryHeap::new();
    que.push(Reverse(0));
    let mut ans = vec![INF; K + 2];

    for i in 1..=K+1 {
        let Reverse(mut cur) = que.pop().unwrap();
        while ans[i-1] == cur {
            cur = que.pop().unwrap().0;
        }
        ans[i] = cur;
        for &a in &A {
            let nxt = cur + a;
            que.push(Reverse(nxt));
        }
    }

    println!("{}", ans[K+1]);
}
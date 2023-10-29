//         B - Yellow and Red Card         
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc292/tasks/abc292_b
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

// solve
fn main() {
    input! {
        N: usize,
        Q: usize,
        query: [(usize, Usize1); Q],
    }

    let mut state = vec![0; N];

    // クエリの処理
    for &(q, x) in &query {
        match q {
            1 => {
                state[x] += 1;
            },
            2 => {
                state[x] += 2;
            },
            3 => {
                if state[x] < 2 {
                    println!("No");
                } else {
                    println!("Yes");
                }
            },
            _ => (),
        }
    }
}

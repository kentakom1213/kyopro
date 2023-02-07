//       B - Playing Cards Validation      
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc277/tasks/abc277_b
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
        mut S: [String; N],
    }

    S.sort();

    let mut is_ok = true;
    for i in 0..N {
        if i > 0 {
            is_ok &= S[i-1] != S[i];
        }
        is_ok &= match &S[i][..1] {
            "H" | "D" | "C" | "S" => true,
            _ => false,
        };
        is_ok &= match &S[i][1..] {
            "A"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9"|"T"|"J"|"Q"|"K" => true,
            _ => false,
        };
    }

    if is_ok {
        println!("Yes");
    } else {
        println!("No");
    }
}

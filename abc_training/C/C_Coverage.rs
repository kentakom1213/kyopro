//               C - Coverage              
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc289/tasks/abc289_c
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
        M: usize,
    }

    let mut sets = vec![];
    for _ in 0..M {
        let mut st: usize = 0;
        input!{c: usize}
        for _ in 0..c {
            input!{v: Usize1}
            st |= 1 << v;
        }
        sets.push(st);
    }

    // bit全探索
    let mut cnt = 0;

    let ok = (1 << N) - 1;
    for i in 0..1<<M {
        let mut tmp = 0;
        for j in 0..M {
            if (i >> j) & 1 == 1 {
                tmp |= sets[j];
            }
        }
        if tmp == ok {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}

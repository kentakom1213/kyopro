//        D - Takahashi's Solitaire
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc277/tasks/abc277_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
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

#[derive(Debug, Clone, Copy)]
struct Count {
    start: usize,
    end: usize,
    sum: usize,
}

/// ランレングス圧縮（+1まで許す）
fn complession(arr: &[usize]) -> Vec<Count> {
    let mut res = vec![];
    let mut cnt = Count {
        start: arr[0],
        end: arr[0],
        sum: arr[0],
    };
    for &val in &arr[1..] {
        if val <= cnt.end + 1 {
            cnt.sum += val;
            cnt.end = val;
        } else {
            res.push(cnt);
            cnt.start = val;
            cnt.end = val;
            cnt.sum = val;
        }
    }
    cnt.end = *arr.last().unwrap();
    res.push(cnt);
    res
}

/// ## 方針
/// 0 ~ M のうち、連続する成分を調べる
/// M -> 0 は0を残しても問題ないため調べなくても良い
fn main() {
    input! {
        N: usize,
        M: usize,
        mut A: [usize; N],
    }
    A.sort();

    let mut comp = complession(&A);

    if comp.last().unwrap().end == M - 1 && comp[0].start == 0 {
        comp.push(Count {
            start: 0,
            end: 0,
            sum: comp.last().unwrap().sum + comp[0].sum,
        });
    }

    let sum = A.iter().sum::<usize>();
    let max = comp.iter().map(|v| v.sum).max().unwrap();

    println!("{}", sum - max);
}

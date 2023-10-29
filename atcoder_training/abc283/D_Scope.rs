//                D - Scope                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc283/tasks/abc283_d
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
use std::vec;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// ## ord
/// `A`を`0`とするascii文字(`A~Za~z`)のインデックスを返す
fn ord(c: char) -> usize {
    let a = 'a' as u32;
    let c = c as u32;
    (c - a) as usize
}

// solve
fn main() {
    input! {
        S: String
    }

    let mut balls = vec![false; 30];
    let mut stack = vec![];

    for c in S.chars() {
        match c {
            '(' => stack.push('('),
            ')' => {
                while !stack.is_empty() {
                    let top = stack.pop().unwrap();
                    match top {
                        '(' => break,
                        _ => balls[ord(top)] = false,
                    }
                }
            },
            _ => {
                if balls[ord(c)] == true {
                    println!("No");
                    return;
                }
                balls[ord(c)] = true;
                stack.push(c);
            },
        }
    }

    println!("Yes");
}

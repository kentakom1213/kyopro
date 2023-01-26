//                  D - 1                  
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc029/tasks/abc029_d
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
use proconio::{input, fastout, marker::{Chars, Bytes}};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// ## 方針
/// 桁ごとに考える
/// ```non-rust-program
/// 000
/// 001
/// ...
/// 010
/// 011
/// ...
/// 345
/// ```
fn main() {
    input!{ N: usize }
    
    let mut ans = 0;

    let mut d = 1;
    for i in 0..12 {
        let Ni = get_digit(N, i);
        let (l, r) = (N / (d * 10), N % d);
        
        if Ni > 1 {
            ans += (l + 1) * d;
        }
        else if Ni == 1 {
            ans += l * d + r + 1;
        }
        else {
            ans += l * d;
        }

        d *= 10;
    }

    println!("{}", ans);
}

fn div_ceil(a: usize, b: usize) -> usize {
    (a + b - 1) / b
}

/// 桁数を求める
/// - `O(log n)`
fn cnt_digit(mut n: usize) -> usize {
    let mut res = 0;
    while n > 0 {
        res += 1;
        n /= 10;
    }
    res
}

/// nのi桁目の数を求める
/// - `O(1)`
fn get_digit(n: usize, i: usize) -> usize {
    n / 10_usize.pow(i as u32) % 10
}

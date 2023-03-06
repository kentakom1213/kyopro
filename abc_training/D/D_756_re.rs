//                 D - 756                 
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc114/tasks/abc114_d
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

/// ## 方針
/// - 素因数の指数を考える
fn main() {
    input! {
        N: usize,
    }

    let mut cnt = vec![0; N+1];
    for mut n in 2..=N {
        // 素因数分解
        for p in 2..=n {
            while n % p == 0 {
                cnt[p] += 1;
                n /= p;
            }
        }
    }

    // 指数がn-1以上の素因数をカウント
    let num = |n: usize| {
        cnt.iter().filter(|&&v| v >= n-1).count()
    };

    let ans = num(5) * (num(5) - 1) * (num(3) - 2) / 2
        + num(15) * (num(5) - 1)
        + num(25) * (num(3) - 1)
        + num(75);

    println!("{}", ans);
}

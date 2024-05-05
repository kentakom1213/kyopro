//        C - Rotate and Palindrome        
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc286/tasks/abc286_c
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

/// `a > b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmin {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// solve
fn main() {
    input! {
        (N, A, B): (usize, usize, usize),
        S: String,
    }

    let mut ans = INF;

    for i in 0..N {
        let mut deq: VecDeque<char> = S[i..].chars().chain(S[..i].chars()).collect();

        // 左右で異なる値を調べる
        let mut cnt = 0;
        while deq.len() >= 2 {
            let l = deq.pop_front().unwrap();
            let r = deq.pop_back().unwrap();
            if l != r {
                cnt += 1;
            }
        }

        chmin!(
            ans,
            A*i + B*cnt,
        );
    }

    println!("{}", ans);
}

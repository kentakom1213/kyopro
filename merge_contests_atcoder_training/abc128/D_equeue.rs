//                D - equeue               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc128/tasks/abc128_d
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

/// `a < b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmax {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a < $b {
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
const INF: isize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// ## 解説
fn main() {
    input! {
        N: usize,
        K: usize,
        V: [isize; N],
    }

    // 操作A: a回、操作B: b回行うときに得られる値の最大値
    let test = |a: usize, b: usize| -> isize {
        let drop = K - a - b;  // 手持ちの宝石を捨てられる回数
        let mut que = vec![];
        que.extend_from_slice(&V[..a]);
        que.extend_from_slice(&V[N-b..]);
        que.sort();
        for val in que.iter_mut().take(drop) {
            if *val < 0 {
                *val = 0;
            }
        }
        que.iter().sum::<isize>()
    };

    let mut ans = -INF;
    for a in 0..=N {
        for b in 0..=N-a {
            if a + b > K {
                break;
            }
            chmax!(
                ans,
                test(a, b),
            );
        }
    }
    println!("{}", ans);
}

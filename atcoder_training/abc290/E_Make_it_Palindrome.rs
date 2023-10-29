//          E - Make it Palindrome
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc290/tasks/abc290_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

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

fn main() {
    input! {
        N: usize,
        A: [Usize1; N],
    }

    // 全てのペアの合計を求める
    let all_pairs = (1..=N).map(|i| (N - i) * ((i + 1) / 2)).sum::<usize>();
    debug!(all_pairs);

    // インデックスを並べた配列を構築
    let mut PP = vec![VecDeque::new(); N];
    for (i, &a) in A.iter().enumerate() {
        PP[a].push_back(i);
    }

    // いいペアを数える
    let mut good_pairs = 0;
    for P in PP {
        let mut l = 0;
        let mut r = P.len().saturating_sub(1);
        while l < r {
            if P[l] + 1 < N - P[r] {
                good_pairs += (r - l) * (P[l] + 1);
                l += 1;
            } else {
                good_pairs += (r - l) * (N - P[r]);
                r -= 1;
            }
        }
    }

    // 答えを求める
    let ans = all_pairs - good_pairs;
    println!("{}", ans);
}

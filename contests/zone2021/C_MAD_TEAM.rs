//               C - MAD TEAM
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/zone2021/tasks/zone2021_c
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![warn(clippy::default_numeric_fallback)]

use std::cmp::Ordering;

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

const PATTERN: usize = 1 << 5;

fn main() {
    input! {
        N: usize,
        X: [[usize; 5]; N],
    }

    let is_ok = |x: usize| -> bool {
        // 1~5番目に対してx以上であるかを判定する
        let mut cnt = vec![0; PATTERN];
        for i in 0..N {
            let mut tmp = 0;
            for j in 0..5 {
                if X[i][j] >= x {
                    tmp |= 1 << j;
                }
            }
            cnt[tmp] += 1;
        }
        // 全探索
        let mut ok = false;
        for k in 0..PATTERN {
            for j in 0..=k {
                for i in 0..=j {
                    // >= x になるか && 組合せが実現できるか
                    ok |= i | j | k == 0b11111
                        && match (i < j, j < k) {
                            (true, true) => cnt[i] >= 1 && cnt[j] >= 1 && cnt[k] >= 1,
                            (true, false) => cnt[i] >= 1 && cnt[j] >= 2,
                            (false, true) => cnt[i] >= 2 && cnt[k] >= 1,
                            (false, false) => cnt[i] >= 3,
                        };
                }
            }
        }
        ok
    };

    // 解の存在で二分探索
    let mut ok = 0;
    let mut ng = 10_000_000_000;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}

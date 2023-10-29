//            C - Four Variables           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc292/tasks/abc292_c
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

/// ## 方針
/// - 調和級数を用いて数え上げ
/// - 全探索：O(NlogN)
fn main() {
    input! {
        N: usize,
    }

    // cnt[i] := a * b == i を満たす(a, b)の組
    let mut cnt = vec![0; N+1];

    for a in 1..=N {
        for b in 1..=N {
            if a * b > N {break;}
            cnt[a * b] += 1;
        }
    }

    // 数え上げ
    let mut ans = 0;
    for i in 1..N {
        ans += cnt[i] * cnt[N - i];
    }

    println!("{}", ans);
}

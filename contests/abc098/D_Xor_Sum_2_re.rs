//              D - Xor Sum 2
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc098/tasks/arc098_b
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

/// ## 方針
/// - 区間の左端を全探索、右端を高速に列挙
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    // 累積xor、累積和を求めておく。
    let mut sxor = vec![0; N + 1];
    let mut ssum = vec![0; N + 1];
    for i in 0..N {
        sxor[i+1] = sxor[i] ^ A[i];
        ssum[i+1] = ssum[i] + A[i];
    }

    let mut ans = 0;

    // 決め打ち2分探索
    for l in 0..N {
        let (mut ok, mut ng) = (l, N);
        while (ng - ok) > 1 {
            let m = (ok + ng) / 2;
            let is_ok = (sxor[m+1] ^ sxor[l]) == (ssum[m+1] - ssum[l]);
            if is_ok {
                ok = m;
            } else {
                ng = m;
            }
        }
        ans += ok - l + 1;
    }

    println!("{}", ans);
}
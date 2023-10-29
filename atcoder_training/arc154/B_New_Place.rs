//              B - New Place
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc154/tasks/arc154_b
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// main
fn main() {
    input! {
        N: usize,
        S: Bytes,
        T: Bytes,
    }

    if S.iter()
        .sorted()
        .zip(T.iter().sorted())
        .any(|(s, t)| s != t)
    {
        println!("-1");
        return;
    }

    // 判定問題を解く
    // Sのk文字目までを除いた文字列をTに一致させられるか
    let can = |k: usize| -> bool {
        let (mut s, mut t) = (k, 0);
        while s < N && t < N {
            if S[s] == T[t] {
                s += 1;
                t += 1;
            } else {
                t += 1;
            }
        }
        s == N
    };

    let mut ok = N;
    let mut ng = NEG1;
    while ok.wrapping_sub(ng) > 1 {
        let mid = ok.wrapping_add(ng) / 2;
        if can(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}

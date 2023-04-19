//         D - Staircase Sequences         
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc190/tasks/abc190_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// main
fn main() {
    input! {
        N: usize,
    }

    // 2*Nの約数の組の個数を求める
    let mut ans = 0;
    for i in 1..=2*N {
        if i*i > 2*N {
            break;
        }
        if 2*N % i == 0 {
            let a = i;
            let b = 2*N / i;
            if (a + b) % 2 == 1 {
                ans += 2;
            }
        }
    }

    println!("{}", ans);
}
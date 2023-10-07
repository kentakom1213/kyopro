//    086 - Snuke's Favorite Arrays（★5）    
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_ch
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![warn(clippy::default_numeric_fallback)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Usize1}};

// constant
const MOD: usize = 1_000_000_007;
const BIT_LEN: usize = 60;

// main
fn main() {
    input! {
        N: usize,
        Q: usize,
        queries: [(Usize1, Usize1, Usize1, usize); Q],
    }

    let mut ans = 1_usize;

    // 桁ごとに考える
    for b in 0..BIT_LEN {
        let mut tmp = 0;
        // bit全探索
        for n in 0..1 << N {
            let mut is_ok = true;
            for i in 0..Q {
                let (x, y, z, w) = queries[i];
                is_ok &= ((n >> x) & 1) | ((n >> y) & 1) | ((n >> z) & 1) == (w >> b) & 1;
            }
            // このビット列iが条件を満たすとき
            if is_ok {
                tmp += 1;
            }
        }
        ans = ans * tmp % MOD;
    }

    println!("{ans}");
}

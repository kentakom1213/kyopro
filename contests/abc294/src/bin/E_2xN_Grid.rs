//               E - 2xN Grid
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc294/tasks/abc294_e
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
        L: usize,
        N: usize,
        M: usize,
        A: [(usize, usize); N],
        B: [(usize, usize); M],
    }

    let mut ans = 0;

    let (mut i, mut j) = (0, 0);
    let (mut asum, mut bsum) = (0, 0);
    let (alen, blen) = (A.len(), B.len());

    // マージ操作
    loop {
        // println!("{}: {}", i, "]".repeat(asum));
        // println!("{}: {}", j, "]".repeat(bsum));
        match (i < alen, j < blen) {
            (true, true) => {
                let (aval, acnt) = A[i];
                let (bval, bcnt) = B[j];
                if aval == bval {
                    let left = asum.max(bsum);
                    let right = if asum + acnt < bsum + bcnt {
                        asum + acnt
                    } else {
                        bsum + bcnt
                    };
                    ans += right.saturating_sub(left);
                }
                if asum + acnt < bsum + bcnt {
                    i += 1;
                    asum += acnt;
                } else {
                    j += 1;
                    bsum += bcnt;
                }
            }
            (true, false) => {
                let (aval, acnt) = A[i];
                let left = asum.max(bsum);
                let right = (asum + acnt).min(bsum);
                ans += right.saturating_sub(left);
                i += 1;
                asum += acnt;
            }
            (false, true) => {
                let (bval, bcnt) = B[j];
                let left = asum.max(bsum);
                let right = (asum).min(bsum + bcnt);
                ans += right.saturating_sub(left);
                j += 1;
                bsum += bcnt;
            }
            (false, false) => break,
        }
        // dbg!(ans);
    }

    println!("{}", ans);
}

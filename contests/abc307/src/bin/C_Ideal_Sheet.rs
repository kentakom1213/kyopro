//             C - Ideal Sheet
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc307/tasks/abc307_c
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::mem::swap;

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

// main
fn main() {
    input! {
        ha: usize,
        wa: usize,
        A: [Chars; ha],
        hb: usize,
        wb: usize,
        B: [Chars; hb],
        hx: usize,
        wx: usize,
        X: [Chars; hx],
    }

    for ar in 0..20 {
        for ac in 0..20 {
            for br in 0..20 {
                for bc in 0..20 {
                    // 重ねる配列
                    let mut Y = vec![vec!['.'; 30]; 30];

                    // Aを重ねる
                    for r in 0..ha {
                        for c in 0..wa {
                            if A[r][c] == '#' {
                                Y[ar + r][ac + c] = '#';
                            }
                        }
                    }

                    // Bを重ねる
                    for r in 0..hb {
                        for c in 0..wb {
                            if B[r][c] == '#' {
                                Y[br + r][bc + c] = '#';
                            }
                        }
                    }

                    // 判定
                    let mut is_ok = true;

                    for i in 0..30 {
                        for j in 0..30 {
                            if 10 <= i && i < 10 + hx && 10 <= j && j < 10 + wx {
                                is_ok &= X[i - 10][j - 10] == Y[i][j];
                            } else {
                                is_ok &= Y[i][j] == '.';
                            }
                        }
                    }

                    if is_ok {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }

    println!("No");
}

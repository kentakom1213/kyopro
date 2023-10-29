//           D - Magical Cookies
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc315/tasks/abc315_d
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
use std::collections::VecDeque;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

/// ## ord
/// `a`を`0`とするascii文字(`A~Za~z`)のインデックスを返す
fn ord(c: char) -> usize {
    let a = 'a' as u32;
    let c = c as u32;
    (c - a) as usize
}

// 文字の種類
const SIGMA: usize = 26;

fn main() {
    input! {
        H: usize,
        W: usize,
        C: [Chars; H],
    }

    // 行、列が持っている色を保存
    let mut rows = vec![[0; SIGMA]; H];
    let mut cols = vec![[0; SIGMA]; W];

    for i in 0..H {
        for j in 0..W {
            let c = ord(C[i][j]);
            rows[i][c] += 1;
            cols[j][c] += 1;
        }
    }

    // 残っている行、列の数
    let mut cnt_row = H;
    let mut cnt_col = W;
    // その行、列が残っているか
    let mut deleted_row = vec![false; H];
    let mut deleted_col = vec![false; W];

    // 行、列を探索
    for _ in 0..H + W {
        let mut nr = vec![];
        let mut nc = vec![];

        for i in 0..H {
            if deleted_row[i] {
                continue;
            }
            for c in 0..SIGMA {
                if rows[i][c] == cnt_col && cnt_col >= 2 {
                    nr.push((i, c));
                }
            }
        }

        for j in 0..W {
            if deleted_col[j] {
                continue;
            }
            for c in 0..SIGMA {
                if cols[j][c] == cnt_row && cnt_row >= 2 {
                    nc.push((j, c));
                }
            }
        }

        for &(i, c) in &nr {
            // 行を削除
            deleted_row[i] = true;
            // すべての行に関して処理
            for j in 0..W {
                if cols[j][c] > 0 {
                    cols[j][c] -= 1;
                }
            }
            cnt_row -= 1;
        }

        for &(j, c) in &nc {
            // 列を削除
            deleted_col[j] = true;
            // すべての列に関して処理
            for i in 0..H {
                if rows[i][c] > 0 {
                    rows[i][c] -= 1;
                }
            }
            cnt_col -= 1;
        }
    }

    println!("{}", cnt_row * cnt_col);
}

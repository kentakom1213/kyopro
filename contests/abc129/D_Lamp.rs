//                 D - Lamp                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc129/tasks/abc129_d
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
use superslice::Ext;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

fn debug_2d<T: std::fmt::Debug>(array: &Vec<Vec<T>>) {
    #[cfg(debug_assertions)]
    for row in array {
        println!("{:?}", row);
    }
}

/// `a < b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmax {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// main
fn main() {
    input! {
        H: usize,
        W: usize,
        S: [Chars; H],
    }

    // Sを転置したもの
    let T = (0..W).map(|c| (0..H).map(|r| S[r][c]).collect_vec()).collect_vec();

    debug_2d(&S);
    debug_2d(&T);

    // 行ごとに累積和をとる
    let mut SS = vec![vec![0; W + 1]; H];
    for r in 0..H {
        for c in 0..W {
            SS[r][c + 1] = SS[r][c] + if S[r][c] == '#' { 1 } else { 0 };
        }
    }

    let mut TT = vec![vec![0; H + 1]; W];
    for c in 0..W {
        for r in 0..H {
            TT[c][r + 1] = TT[c][r] + if T[c][r] == '#' { 1 } else { 0 };
        }
    }

    debug_2d(&SS);
    debug_2d(&TT);

    // 答え
    let mut ans = 0;
    for r in 0..H {
        for c in 0..W {
            // 横方向
            let cnt_x = SS[r][c + 1];
            let left = SS[r].lower_bound(&cnt_x);
            let right = SS[r].upper_bound(&cnt_x);
            let width = right - left - 1;

            // 縦方向
            let cnt_y = TT[c][r + 1];
            let bottom = TT[c].lower_bound(&cnt_y);
            let top = TT[c].upper_bound(&cnt_y);
            let height = top - bottom - 1;

            debug!(r, c, cnt_x, cnt_y, left, right, bottom, top, width, height);

            chmax!(
                ans,
                (width + height).saturating_sub(1)
            );
        }
    }

    println!("{}", ans);
}

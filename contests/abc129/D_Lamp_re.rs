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

    // 縦方向のDP
    let mut dp_x = vec![vec![0; W]; H];

    for i in 0..H {
        for j in 0..W {
            if S[i][j] == '#' {
                continue;
            }
            dp_x[i][j] = 1;
            if j > 0 {
                dp_x[i][j] += dp_x[i][j - 1];
            }
        }
        for j in (0..W - 1).rev() {
            if dp_x[i][j] == 0 {
                continue;
            }
            chmax!(
                dp_x[i][j],
                dp_x[i][j + 1],
            );
        }
    }

    // 横方向のDP
    let mut dp_y = vec![vec![0; W]; H];

    for j in 0..W {
        for i in 0..H {
            if S[i][j] == '#' {
                continue;
            }
            dp_y[i][j] = 1;
            if i > 0 {
                dp_y[i][j] += dp_y[i - 1][j];
            }
        }
        for i in (0..H - 1).rev() {
            if dp_y[i][j] == 0 {
                continue;
            }
            chmax!(
                dp_y[i][j],
                dp_y[i + 1][j],
            );
        }
    }
    

    debug_2d(&dp_x);
    debug_2d(&dp_y);

    // 答え
    let mut ans = 0;
    for i in 0..H {
        for j in 0..W {
            chmax!(
                ans,
                dp_x[i][j] + dp_y[i][j] - 1,
            );
        }
    }

    println!("{}", ans);
}

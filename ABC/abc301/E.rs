// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
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
const NEG1: usize = 1_usize.wrapping_neg();
const MOVE: [(usize, usize); 4] = [
    (1, 1),
    (1, NEG1),
    (NEG1, 1),
    (NEG1, NEG1),
];

// main
fn main() {
    input! {
        H: usize,
        W: usize,
        T: usize,
        A: [Chars; H],
    }

    // スタートとゴールを探索
    let mut S = (0, 0);
    let mut G = (0, 0);
    for i in 0..H {
        for j in 0..W {
            match A[i][j] {
                'S' => S = (i, j),
                'G' => G = (i, j),
                _ => (),
            }
        }
    }

    // dp[t][i][j] := (t回操作を行った後)マス(i,j)にいるとき、訪れたお菓子マスの最大値
    let mut dp = vec![vec![vec![-1; W]; H]; T + 1];

    dp[0][S.0][S.1] = 0;

    for t in 0..T {
        for r in 0..H {
            for c in 0..W {
                for &(dr, dc) in &MOVE {
                    let nr = r.wrapping_add(dr);
                    let nc = c.wrapping_div(dc);
                    if nr >= H || nc >= W { continue; }
                    if A[nr][nc] == '#' { continue; }
                    chmax!(
                        dp[t + 1][nr][nc],
                        dp[t][r][c] + if A[nr][nc] == 'o' { 1 } else { 0 }
                    );
                }
            }
        }
    }

    {
        #[cfg(debug_assertions)]
        for t in &dp {
            for r in t {
                println!("{:?}", r);
            }
            println!();
        }
    }

    let ans = dp[T][G.0][G.1];
    println!("{}", ans);
}
//           E - All-you-can-eat
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc145/tasks/abc145_e
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
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

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
    ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
        chmax! {
            $a,
            ($b).max($c)
            $(,$other)*
        }
    }}
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: isize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        T: usize,
        dish: [(isize, isize); N],
    }

    let (W, V): (Vec<isize>, Vec<isize>) = dish.iter().cloned().unzip();

    // dp1[i][j] := [0,i)個目までの料理を時間jまでに食べ切る時の美味しさの最大値
    let mut dp1 = vec![vec![-INF; T]; N + 1];
    dp1[0][0] = 0;

    for i in 0..N {
        for j in 0..T {
            // 使わない時
            chmax!(dp1[i + 1][j], dp1[i][j], dp1[i + 1][j.saturating_sub(1)]);
            // 使う時
            if j >= W[i] as usize {
                chmax!(dp1[i + 1][j], dp1[i][j - W[i] as usize] + V[i]);
            }
        }
    }

    // dp2[i][j] := [i,N)個目までの料理を時間jまでに食べ切る時の美味しさの最大値
    let mut dp2 = vec![vec![-INF; T]; N + 1];
    dp2[N][0] = 0;

    for i in (1..=N).rev() {
        for j in 0..T {
            // 使わない時
            chmax!(dp2[i - 1][j], dp2[i][j], dp2[i - 1][j.saturating_sub(1)]);
            // 使う時
            if j >= W[i - 1] as usize {
                chmax!(dp2[i - 1][j], dp2[i][j - W[i - 1] as usize] + V[i - 1]);
            }
        }
    }

    // 最後に食べる料理を全探索
    let mut ans = 0;
    for i in 0..N {
        // 料理iを食べない時、時間T-1までに食べ切れる料理の美味しさの総和の最大値
        let mx = (0..T)
            .map(|t| dp1[i][t] + dp2[i + 1][T - t - 1])
            .max()
            .unwrap();
        chmax!(ans, mx + V[i]);
    }

    println!("{}", ans);
}

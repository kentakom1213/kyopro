//               E - Traveler
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc197/tasks/abc197_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::{
    cmp::{max, min, Reverse},
    vec,
};

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

/// `a > b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmin {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a > $b {
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
const INF: isize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// ## 解説
/// - 同じ色で塗られたボールの左端、右端を考えれば良い
/// - 色xについて、x未満のボールを全て拾った上で色xの左端、右端に到達するための
/// 最小の時間をDPで求める
fn main() {
    input! {
        N: usize,
        XC: [(isize, usize); N],
    }
    // ball[i][j] := 色iの(j==0 ? 左 : 右)端の要素の座標
    let mut ball = vec![vec![0; 2]; N + 1];

    // ballの初期化
    for i in 1..=N {
        ball[i][0] = INF;
        ball[i][1] = -INF;
    }

    for &(x, c) in &XC {
        chmin! {
            ball[c][0],
            x
        };
        chmax! {
            ball[c][1],
            x
        };
    }

    // dp[i][j] := i以下の色を全て回収したのち、色iの(j==0 ? 左 : 右)端に到達するための時間の最小値
    let mut dp = vec![vec![INF; 2]; N + 1];
    dp[0][0] = 0;
    dp[0][1] = 0;

    // dpの更新
    for c in 1..=N {
        // cの左端から右端までの距離
        let dist = ball[c][1] - ball[c][0];

        // cのボールが存在しないとき
        if dist < 0 {
            dp[c][0] = dp[c - 1][0];
            dp[c][1] = dp[c - 1][1];
            ball[c][0] = ball[c - 1][0];
            ball[c][1] = ball[c - 1][1];
            continue;
        }

        // c-1の左端 → cの左端
        chmin! {
            dp[c][0],
            dp[c-1][0] + (ball[c-1][0] - ball[c][1]).abs() + dist
        };
        // c-1の左端 → cの右端
        chmin! {
            dp[c][1],
            dp[c-1][0] + (ball[c-1][0] - ball[c][0]).abs() + dist
        };
        // c-1の右端 → cの左端
        chmin! {
            dp[c][0],
            dp[c-1][1] + (ball[c-1][1] - ball[c][1]).abs() + dist
        };
        // c-1の右端 → cの右端
        chmin! {
            dp[c][1],
            dp[c-1][1] + (ball[c-1][1] - ball[c][0]).abs() + dist
        };
    }

    // 出力
    let ans = min(dp[N][0] + ball[N][0].abs(), dp[N][1] + ball[N][1].abs());

    println!("{}", ans);
}

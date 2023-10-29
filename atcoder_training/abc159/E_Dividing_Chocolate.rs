//          E - Dividing Chocolate
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc159/tasks/abc159_e
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
use superslice::Ext;

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
const UINF: usize = 1001001001001001001;
const IINF: isize = 1001001001001001001;

// main
fn main() {
    input! {
        H: usize,
        W: usize,
        K: usize,
        S: [Chars; H],
    }

    let mut choco = vec![vec![0; W]; H];

    // Sを数字に変換
    for i in 0..H {
        for j in 0..W {
            choco[i][j] = S[i][j].to_digit(10).unwrap() as usize;
        }
    }

    // 答え
    let mut ans = UINF;

    // 縦の全探索
    for i in 0..1 << H - 1 {
        // 切る線を列挙
        let mut split = (0..H - 1).filter(|j| i >> j & 1 == 1).collect_vec();

        // 線の本数の合計
        let mut tmp = split.len();

        // 番兵の追加
        split.push(H - 1);

        // 分割された列に含まれるホワイトチョコレートの累積
        let mut acc = vec![vec![0; W + 1]; split.len()];

        // 列ごとにまとめる
        for r in 0..H {
            for c in 0..W {
                // 加える行
                let row = split.lower_bound(&r);
                acc[row][c + 1] += choco[r][c];
            }
        }

        // 累積和を取る
        for row in 0..acc.len() {
            for c in 0..W {
                acc[row][c + 1] += acc[row][c];
            }
        }

        // dp[i] := i列目までが条件を満たすとき、i列目までに横に割る回数の合計の最小値
        let mut dp = vec![IINF; W + 1];
        dp[0] = -1;

        // 更新
        for c in 1..=W {
            let mut most_right = 0;
            // 各行について、条件を満たす行の中で中で最も右に有るもの
            for row in 0..split.len() {
                chmax!(
                    most_right,
                    acc[row].lower_bound(&(acc[row][c]).saturating_sub(K))
                );
            }
            // 更新
            dp[c] = dp[most_right] + 1;
        }

        if cfg!(debug_assertions) {
            println!("{:?}", split);
            for r in &acc {
                println!("{:?}", r);
            }
            println!("{:?}", dp);
        }

        // 線の本数の更新
        tmp += dp[W] as usize;

        // 解の更新
        chmin!(ans, tmp);
    }

    println!("{}", ans);
}

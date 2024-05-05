// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        M: usize,
        S: Chars,
        T: Chars,
    }

    // dp[i][j] := Sのi番目をTのj番目にするようなスタンプの置き方はあるか
    let mut dp = vec![vec![false; M]; N];

    if S[0] == T[0] {
        dp[0][0] = true;
    }

    for i in 1..N {
        for j in 0..M {
            if dp[i - 1][j] && S[i - 1] == T[j] {
                if j < M - 1 {
                    // 最後の文字以外のとき（直前が一致している場合に限る）
                    if S[i] == T[j + 1] {
                        dp[i][j + 1] = true;
                    }
                } else {
                    // 最後の文字が一致しているとき，Tのどの文字からでも置き始められる
                    for k in 0..M {
                        // Tのk文字目から始める場合
                        if k <= i && i <= N - M + k && S[i] == T[k] {
                            dp[i][k] = true;
                        }
                    }
                }
                // 新しく文字を置き始める
                if i <= N - M && S[i] == T[0] {
                    dp[i][0] = true;
                }
            }
        }
    }

    if cfg!(debug_assertions) {
        for row in &dp {
            eprintln!("{:?}", row);
        }
    }

    if dp[N - 1][M - 1] {
        println!("Yes");
    } else {
        println!("No");
    }
}

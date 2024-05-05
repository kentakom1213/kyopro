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

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

/// `chmin!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最小のものを、`x1`に代入する
/// - 代入があったとき、`true`を返す
#[macro_export]
macro_rules! chmin {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
    ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
        chmin! {
            $a,
            ($b).min($c)
            $(,$other)*
        }
    }};
}

const INF: usize = 1001001001001001001;

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize; N],
    }

    // dp[i][j][k] := Aのi番目に対してBi==kのとき総和jを実現するための操作回数の最小値
    let mut dp = vec![vec![[INF; 2]; M + 1]; N + 1];

    // 1 -> 0の更新のときに操作が加わる
    dp[0][0][0] = 1;
    dp[0][0][1] = 0;

    for i in 0..N {
        for j in 0..=M {
            // 0 -> 0
            chmin! {
                dp[i + 1][j][0],
                dp[i][j][0]
            };
            // 0 -> 1
            if j >= A[i] {
                chmin! {
                    dp[i + 1][j][1],
                    dp[i][j - A[i]][0]
                };
            }
            // 1 -> 0
            chmin! {
                dp[i + 1][j][0],
                dp[i][j][1] + 1
            };
            // 1 -> 1
            if j >= A[i] {
                chmin! {
                    dp[i + 1][j][1],
                    dp[i][j - A[i]][1]
                };
            }
        }
    }

    if cfg!(debug_assertions) {
        for row in &dp {
            eprintln!("{:?}", row);
        }
    }

    // 出力
    for j in 1..=M {
        let ans = *dp[N][j].iter().min().unwrap();

        if ans == INF {
            println!("-1");
        } else {
            println!("{}", ans);
        }
    }
}

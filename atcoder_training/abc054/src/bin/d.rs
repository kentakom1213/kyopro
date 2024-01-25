#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}
macro_rules! debug2D {
    ( $array:expr ) => {{
        #![cfg(debug_assertions)]
        eprintln!("{}: ", stringify!($array));
        for row in &$array {
            eprintln!("{:?}", row);
        }
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

use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};
use rustc_hash::FxHashMap;

fn main() {
    input! {
        N: usize,
        Ma: usize,
        Mb: usize,
        ABC: [(usize, usize, usize); N]
    }

    // dp[i][j][k] := i番目までみたとき，選んだ薬品の和が
    //   - 物質A: j
    //   - 物質B: k
    // になっているときのコストの最小値
    let mut dp = vec![vec![vec![INF; 444]; 444]; 44];
    dp[0][0][0] = 0;

    for (i, &(a, b, c)) in ABC.iter().enumerate() {
        for j in 0..444 {
            for k in 0..444 {
                if dp[i][j][k] == INF {
                    continue;
                }
                // 加える
                chmin! {
                    dp[i + 1][j + a][k + b],
                    dp[i][j][k] + c
                };
                // 加えない
                chmin! {
                    dp[i + 1][j][k],
                    dp[i][j][k]
                };
            }
        }
    }

    // 答えを求める
    let mut ans = INF;
    for i in 1.. {
        if i * (Ma + Mb) > 444 {
            break;
        }
        chmin! {
            ans,
            dp[N][i * Ma][i * Mb]
        };
    }

    if ans == INF {
        println!("-1");
    } else {
        println!("{ans}");
    }
}

const INF: usize = 1001001001001001001;

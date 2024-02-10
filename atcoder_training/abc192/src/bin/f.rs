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
        for row in $array {
            eprintln!("{:?}", row);
        }
    }};
}

/// `chmax!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最大のものを、`x1`に代入する
/// - 代入があったとき、`true`を返す
#[macro_export]
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

fn main() {
    input! {
        N: usize,
        X: usize,
        A: [usize; N],
    }

    let mut ans = INF;

    for c in 1..=N {
        // dp[i][j] := iで割ったあまりがjであるような部分集合の最大値
        let mut dp = vec![vec![None; N]; N + 1];
        let mut ndp = vec![vec![None; N]; N + 1];
        dp[0][0] = Some(0);

        for i in 0..N {
            for j in 0..N {
                for k in 0..N {
                    // 選ばないとき
                    chmax! {
                        ndp[j][k],
                        dp[j][k]
                    };
                    // 選ぶとき
                    let nxt = (k + A[i]) % c;
                    chmax! {
                        ndp[j + 1][nxt],
                        dp[j][k].map(|v| v + A[i])
                    };
                }
            }
            (dp, ndp) = (ndp, dp);
        }
        debug2D!(&dp);

        // 何回の操作で手に入れることができるか
        // 初期値
        if let Some(init) = dp[c][X % c] {
            chmin! {
                ans,
                (X - init) / c
            };
        }
    }

    println!("{ans}");
}

const INF: usize = 1001001001001001001;

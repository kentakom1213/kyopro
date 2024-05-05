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

use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};
use rustc_hash::FxHashMap;

fn main() {
    input! {
        N: usize,
        M: usize,
        mut A: [usize; M]
    }

    A.sort();

    // 数と桁数
    let D = vec![INF, 2, 5, 5, 4, 5, 6, 3, 7, 6];

    // dp[i] := i本のマッチを使って作ることのできる数の個数の最大値
    let mut dp = vec![-(INF as isize); N + 1];
    dp[0] = 0;

    for i in 0..=N {
        for &a in A.iter().rev() {
            let d = D[a];
            if i >= d {
                chmax! {
                    dp[i],
                    dp[i - d] + 1
                };
            }
        }
    }

    debug!(dp);

    // dp復元で、各文字数を何個使えるか判定
    let mut S = String::new();
    let mut n = N;

    while n > 0 {
        for &a in A.iter().rev() {
            let d = D[a];
            if n >= d && dp[n - d] + 1 == dp[n] {
                let c = a.to_string();
                S += &c;
                n -= d;
                break;
            }
        }
    }

    println!("{S}");
}

const INF: usize = 1001001001001001001;

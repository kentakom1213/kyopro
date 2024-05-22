// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};
use rustc_hash::FxHashMap;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

/// 参考: <https://tatyam.hatenablog.com/entry/2020/04/13/052313>
fn main() {
    input! {
        N: usize,
        A: [isize; N]
    }

    // 無視できる要素の数
    let ignore = 1 + N % 2;

    let mut dp = FxHashMap::default();

    dp.insert((0, 0), 0);

    for i in 0..=N {
        for j in 0..=ignore {
            // 無視する場合
            if j < ignore {
                chmax! {
                    *dp.entry((i + 1, j + 1)).or_insert(-INF),
                    *dp.get(&(i, j)).unwrap_or(&-INF)
                };
            }
            // 採用する場合
            if i < N {
                chmax! {
                    *dp.entry((i + 2, j)).or_insert(-INF),
                    *dp.get(&(i, j)).unwrap_or(&-INF) + A[i],
                };
            }
        }
    }

    debug!(dp);

    let ans = dp[&(N + 1, ignore)];

    println!("{ans}");
}

const INF: isize = 1001001001001001001;

mod macro_chmax {
    #![allow(dead_code)]
    //! chmaxの実装
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
}

mod macro_debug {
    #![allow(dead_code)]
    //! デバッグ用マクロ
    /// デバッグ用マクロ
    #[macro_export]
    macro_rules! debug {
        ( $($val:expr),* $(,)* ) => {{
            #[cfg(debug_assertions)]
            eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
        }};
    }
    /// 配列用マクロ
    #[macro_export]
    macro_rules! debug2D {
        ( $array:expr ) => {{
            #![cfg(debug_assertions)]
            eprintln!("{}: ", stringify!($array));
            for row in &$array {
                eprintln!("{:?}", row);
            }
        }};
    }
}

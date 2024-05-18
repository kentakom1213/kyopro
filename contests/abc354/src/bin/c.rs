#![allow(non_snake_case)]

use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        mut AC: [(usize, usize); N]
    }

    let X = AC
        .iter()
        .enumerate()
        .sorted_by_key(|(_, &(a, c))| c)
        .collect_vec();

    debug!(X);

    // Aが小さくなる場合は不採用
    let mut ans = vec![];

    let mut a_max = 0;

    for (i, &(a, c)) in X {
        if a_max <= a {
            // 不採用
            ans.push(i + 1);
            a_max = a;
        }
    }

    ans.sort();

    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
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

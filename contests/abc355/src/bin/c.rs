#![allow(non_snake_case)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        T: usize,
        A: [Usize1; T]
    }

    let mut rows = vec![0; N];
    let mut cols = vec![0; N];
    let mut X = vec![0; 2];

    for (i, &a) in A.iter().enumerate() {
        // 印をつける
        let (r, c) = (a / N, a % N);

        rows[r] += 1;
        cols[c] += 1;

        // 対角線
        if r == c {
            X[0] += 1;
        }
        if r == N - c - 1 {
            X[1] += 1;
        }

        // 判定
        if rows[r] == N || cols[c] == N || X[0] == N || X[1] == N {
            println!("{}", i + 1);
            return;
        }
    }

    println!("-1");
}

const _INF: usize = 1001001001001001001;

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

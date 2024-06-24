#![allow(non_snake_case)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        A: [Usize1; 2 * N]
    }

    let mut cnt = 0;

    for i in 0..2 * N - 2 {
        if A[i] == A[i + 2] {
            cnt += 1;
        }
    }

    println!("{cnt}");
}

const INF: usize = 1001001001001001001;

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

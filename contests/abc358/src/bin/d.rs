#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        mut A: [usize; N],
        mut B: [usize; M],
    }

    A.sort();
    B.sort();

    let mut ans = 0;

    let mut i = 0;
    let mut j = 0;

    while i < N && j < M {
        debug!(i, j, A[i], B[j]);
        if B[j] <= A[i] {
            ans += A[i];
            i += 1;
            j += 1;
        } else if B[j] > A[i] {
            i += 1;
        }
    }

    if j == M {
        println!("{ans}");
    } else {
        println!("-1");
    }
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

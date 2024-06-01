#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize; M],
        X: [[usize; M]; N]
    }

    let mut S = vec![0; M];

    for i in 0..N {
        for j in 0..M {
            S[j] += X[i][j];
        }
    }

    let mut isok = true;

    for i in 0..M {
        isok &= S[i] >= A[i];
    }

    if isok {
        println!("Yes");
    } else {
        println!("No");
    }
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

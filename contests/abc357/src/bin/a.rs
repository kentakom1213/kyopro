#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        mut M: usize,
        H: [usize; N],
    }

    let mut ok = 0;

    for &h in &H {
        if h > M {
            break;
        }
        M -= h;
        ok += 1;
    }

    println!("{ok}");
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

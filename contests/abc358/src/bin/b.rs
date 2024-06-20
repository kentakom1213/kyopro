#![allow(non_snake_case)]

use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        N: usize,
        A: usize,
        T: [usize; N]
    }

    let mut t = 0_usize;

    for i in 0..N {
        let wait = t.saturating_sub(T[i]);
        t = wait + T[i] + A;

        println!("{t}");
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

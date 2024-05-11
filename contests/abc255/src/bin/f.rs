#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        P: [usize; N],
        I: [usize; N],
    }

    let mut cur = INF;

    let mut ans = vec![[INF, INF]; N];
    let mut stack = vec![];

    let mut i = 0;
    let mut j = 0;

    while i < N && j < N {
        
    }

    debug2D!(ans);
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

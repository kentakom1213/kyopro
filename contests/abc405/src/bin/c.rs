#![allow(non_snake_case)]

use crate::cp_library_rs::debug;
use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    let sum: usize = A.iter().sum();

    debug!(sum);

    let diag: usize = A.iter().map(|&a| a * a).sum();

    let ans = (sum * sum - diag) / 2;

    println!("{ans}");
}

// ==================== cp-library-rs ====================
pub mod cp_library_rs {
    #![allow(dead_code)]
    pub mod debug {
        /// 変数をデバッグ出力する
        #[macro_export]
        macro_rules! debug {
            ( $($val:expr),* $(,)* ) => {{
                #[cfg(debug_assertions)]
                eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
            }};
        }
    }
}

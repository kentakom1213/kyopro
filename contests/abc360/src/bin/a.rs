#![allow(non_snake_case)]

use proconio::{input, marker::Chars};

use crate::yesno::YesNo;

fn main() {
    input! {
        S: Chars
    }

    let mut r = 0;
    let mut m = 0;
    for i in 0..3 {
        if S[i] == 'R' {
            r = i;
        }
        if S[i] == 'M' {
            m = i;
        }
    }

    println!("{}", (r < m).yesno());
}

const INF: usize = 1001001001001001001;

mod yesno {
    #![allow(dead_code)]
    //! boolから"Yes"/"No"への変換
    pub trait YesNo {
        /// `true`->`"Yes"`, `false`->`"No"` に変換
        fn yesno(&self) -> String;
    }
    impl YesNo for bool {
        fn yesno(&self) -> String {
            if *self {
                "Yes".to_string()
            } else {
                "No".to_string()
            }
        }
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

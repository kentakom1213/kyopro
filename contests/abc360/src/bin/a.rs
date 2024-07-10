#![allow(non_snake_case)]

use crate::cp_library_rs::yesno::YesNo;
use proconio::{input, marker::Chars};

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

// ==================== cp-library-rs ====================
mod cp_library_rs {
    #![allow(dead_code)]
    pub mod yesno {
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
}

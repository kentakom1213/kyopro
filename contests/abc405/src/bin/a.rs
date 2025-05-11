#![allow(non_snake_case)]

use crate::cp_library_rs::utils::yesno::YesNo;
use proconio::input;

fn main() {
    input! {
        R: usize,
        X: usize,
    }

    let israted = X == 1 && (1600 <= R && R <= 2999) || X == 2 && (1200 <= R && R <= 2399);

    println!("{}", israted.yesno());
}

// ==================== cp-library-rs ====================
pub mod cp_library_rs {
    #![allow(dead_code)]
    pub mod utils {
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
}

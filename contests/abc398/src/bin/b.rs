#![allow(non_snake_case)]

use crate::cp_library_rs::utils::yesno::YesNo;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        A: [Usize1; 7],
    }

    let C = A.iter().fold([0; 13], |mut x, &a| {
        x[a] += 1;
        x
    });

    let mut is_ok = false;

    for i in 0..13 {
        for j in i + 1..13 {
            is_ok |= C[i] >= 2 && C[j] >= 3;
            is_ok |= C[i] >= 3 && C[j] >= 2;
        }
    }

    println!("{}", is_ok.yesno());
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

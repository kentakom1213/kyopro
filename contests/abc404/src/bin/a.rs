#![allow(non_snake_case)]

use crate::cp_library_rs::string::consts::ASCII_LOWERCASE_STR;
use proconio::input;

fn main() {
    input! {
        S: String
    }

    for c in ASCII_LOWERCASE_STR.chars() {
        if !S.contains(c) {
            println!("{c}");
            return;
        }
    }
}

// ==================== cp-library-rs ====================
pub mod cp_library_rs {
    #![allow(dead_code)]
    pub mod string {
        pub mod consts {
            //! 文字列関係の定数
            /// 英小文字（文字列）
            pub const ASCII_LOWERCASE_STR: &str = "abcdefghijklmnopqrstuvwxyz";
            /// 英小文字（配列)
            pub const ASCII_LOWERCASE_ARR: [char; 26] = [
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
            ];
            /// 英大文字（文字列）
            pub const ASCII_UPPERCASE_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
            /// 英大文字（配列）
            pub const ASCII_UPPERCASE_ARR: [char; 26] = [
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
            ];
            /// 16進数の文字（小文字）
            pub const HEX_LOWER: [char; 16] = [
                '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
            ];
            /// 16進数の文字（大文字）
            pub const HEX_UPPER: [char; 16] = [
                '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
            ];
        }
    }
}

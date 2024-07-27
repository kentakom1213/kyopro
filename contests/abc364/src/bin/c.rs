#![allow(non_snake_case)]

use crate::cp_library_rs::{chmin, utils::consts::INF};
use proconio::input;

fn main() {
    input! {
        N: usize,
        X: usize,
        Y: usize,
        mut A: [usize; N],
        mut B: [usize; N],
    }

    let mut ans = N;

    // Aで見る
    A.sort();
    A.reverse();
    let mut tmp = 0;
    for (i, &a) in (1..).zip(&A) {
        tmp += a;
        if tmp > X {
            chmin! {
                ans,
                i
            };
        }
    }

    // Bでみる
    B.sort();
    B.reverse();
    tmp = 0;
    for (i, &b) in (1..).zip(&B) {
        tmp += b;
        if tmp > Y {
            chmin! {
                ans,
                i
            };
        }
    }

    println!("{ans}");
}

// ==================== cp-library-rs ====================
mod cp_library_rs {
    #![allow(dead_code)]
    pub mod utils {
        pub mod consts {
            //! 定数
            /// MOD用の定数：$`998244353`$
            pub const MOD998: usize = 998244353;
            /// MOD用の定数：$`10^9 + 7`$
            pub const MOD107: usize = 1000000007;
            /// 十分大きい数（usize）
            pub const INF: usize = 1001001001001001001;
            /// 十分大きい数（isize）
            pub const IINF: isize = 1001001001001001001;
            /// usizeにおける`-1`の値
            pub const NEG1: usize = 1_usize.wrapping_neg();
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
    pub mod chmin {
        /// `chmin!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最小のものを、`x1`に代入する
        /// - 代入があったとき、`true`を返す
        #[macro_export]
        macro_rules! chmin {
            ( $a:expr, $b:expr $(,)* ) => {{
                if $a > $b {
                    $a = $b;
                    true
                } else {
                    false
                }
            }};
            ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
                chmin! {
                    $a,
                    ($b).min($c)
                    $(,$other)*
                }
            }};
        }
    }
}

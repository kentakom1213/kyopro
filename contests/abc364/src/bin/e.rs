#![allow(non_snake_case)]

use crate::cp_library_rs::{chmin, debug, debug2D, utils::consts::INF};
use proconio::input;

fn main() {
    input! {
        N: usize,
        X: usize,
        Y: usize,
        AB: [(usize, usize); N]
    }

    // dp[i][j] := i個選び，選んだ料理のAの合計がjのときのBの合計の最小値
    let mut dp = vec![vec![INF; X + 10]; N + 1];

    dp[0][0] = 0;

    for &(a, b) in &AB {
        let mut nxt = vec![vec![INF; X + 10]; N + 1];

        for i in 0..N {
            for Sa in 0..X {
                // 食べない
                chmin! {
                    nxt[i][Sa],
                    dp[i][Sa],
                };
                // 食べる
                if Sa + a <= X {
                    chmin! {
                        nxt[i + 1][Sa + a],
                        dp[i][Sa] + b,
                    };
                }
            }
        }
        dp = nxt;
    }

    // Y未満を見つける
    let mut ans = 0;
    for i in 0..=N {
        for j in 0..=X {
            if dp[i][j] <= Y {
                ans = i;
            }
        }
    }

    ans = N.min(ans + 1);

    println!("{ans}");
}

const MAX: usize = 10101;

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
    pub mod debug2D {
        /// 2次元配列をデバッグ出力する
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
}

#![allow(non_snake_case)]

use crate::cp_library_rs::{debug, utils::consts::NEG1};
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        N: usize,
        Q: usize,
        mut A: [isize; N],
        BK: [(isize, usize); Q]
    }

    // ソート
    A.sort();

    for &(b, k) in &BK {
        // 距離ok以下の点がk個未満である
        let mut ng = -1;
        let mut ok = 1001001001;
        while ok - ng > 1 {
            let mid = (ng + ok) / 2;

            // debug!(b, mid);

            // 左側
            let l = b - mid;
            let il = A.lower_bound(&l);

            // debug!(l, il);

            // 右側
            let r = b + mid;
            let ir = A.upper_bound(&r);

            // debug!(r, ir);

            let cnt = ir - il;

            // debug!(cnt);

            if cnt < k {
                ng = mid;
            } else {
                ok = mid;
            }
        }

        println!("{ok}");
    }
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

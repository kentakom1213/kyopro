#![allow(non_snake_case)]

use crate::cp_library_rs::{debug, suffix_array::SuffixArray};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut S: String,
        Q: usize,
        T: [String; Q]
    }

    S.push('~');

    // suffix arrayを構築（愚直）
    let SA = SuffixArray::build(&S);

    debug!(SA);

    // 二分探索
    for t in T {
        let ok = SA.partition_point(|&i| &S[i..] <= &t[..]);

        let tt = t + "~";
        let ng = SA.partition_point(|&i| &S[i..] <= &tt[..]);

        println!("{}", ng - ok);
    }
}
// ==================== cp-library-rs ====================
mod cp_library_rs {
    #![allow(dead_code)]
    pub mod cfor {
        /// Cスタイルのfor文
        /// ```text
        /// cfor! {define ; finish ; increment ;; {
        ///     block
        /// }}
        /// ```
        #[macro_export]
        macro_rules! cfor {
            ($def:stmt ; $fin:expr ; $incr:stmt ;; $bl:block) => {{
                $def
                while $fin {
                    $bl
                    $incr
                }
            }}
        }
    }
    pub mod suffix_array {
        //! 接尾辞配列（Manber-Myers法）
        use crate::{cfor, debug};

        /// SuffixArray
        pub struct SuffixArray;

        impl SuffixArray {
            /// (rank[i], rank[i+k]) により比較する
            fn ord(rank: &[isize], i: usize, k: usize) -> (isize, isize) {
                (rank[i], *rank.get(i + k).unwrap_or(&-1))
            }

            /// 文字列Sの接尾辞配列を構築し，返す
            /// - 計算量：`O(|S| log^2 |S|)`
            pub fn build(S: &str) -> Vec<usize> {
                let N = S.len();
                let mut rank: Vec<_> = S.chars().map(|c| c as isize).collect();
                // 末尾（空文字列）
                rank.push(-1);
                let mut sa: Vec<_> = (0..=N).collect();

                cfor! {let mut k = 1; k <= N; k *= 2;; {
                    // rankでソート
                    sa.sort_by_key(|&i| Self::ord(&rank, i, k));
                    // rankの更新
                    let mut tmp = vec![0; N + 1];
                    tmp[sa[0]] = 0;
                    for i in 1..=N {
                        tmp[sa[i]] = tmp[sa[i - 1]] +
                            (Self::ord(&rank, sa[i - 1], k) < Self::ord(&rank, sa[i], k)) as isize;
                    }
                    rank = tmp;
                }}
                sa
            }
        }
    }
    pub mod debug {
        /// デバッグ用マクロ
        #[macro_export]
        macro_rules! debug {
            ( $($val:expr),* $(,)* ) => {{
                #[cfg(debug_assertions)]
                eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
            }};
        }
    }
}

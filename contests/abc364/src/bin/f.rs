#![allow(non_snake_case)]

use std::collections::BTreeSet;

use crate::cp_library_rs::{chmax, debug};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        Q: usize,
        mut LRC: [(Usize1, Usize1, usize); Q]
    }

    if !is_spanning(N, &LRC) {
        println!("-1");
        return;
    }

    // まだ結ばれていないN側の頂点
    let mut set = BTreeSet::from_iter(0..N);

    // 重みが小さい方から貪欲に取る
    LRC.sort_by_key(|&(_, _, c)| c);

    let mut ans = 0;

    for &(l, r, c) in &LRC {
        let rm = set.range(l..=r).cloned().collect_vec();

        ans += c * (rm.len() + 1);

        // 削除
        for &v in &rm {
            set.remove(&v);
        }
    }

    // 最初の一つはいらない
    ans -= LRC[0].2;

    println!("{ans}");
}

/// 全域性の判定
fn is_spanning(N: usize, LRC: &[(usize, usize, usize)]) -> bool {
    let mut seen = 0;
    for &(l, r, _) in LRC.iter().sorted() {
        if l <= seen {
            chmax!(seen, r);
        } else {
            return false;
        }
    }
    seen >= N - 1
}

// ==================== cp-library-rs ====================
mod cp_library_rs {
    #![allow(dead_code)]
    pub mod chmax {
        /// `chmax!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最大のものを、`x1`に代入する
        /// - 代入があったとき、`true`を返す
        #[macro_export]
        macro_rules! chmax {
            ( $a:expr, $b:expr $(,)* ) => {{
                if $a < $b {
                    $a = $b;
                    true
                } else {
                    false
                }
            }};
            ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
                chmax! {
                    $a,
                    ($b).max($c)
                    $(,$other)*
                }
            }}
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

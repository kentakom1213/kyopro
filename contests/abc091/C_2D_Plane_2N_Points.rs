//          C - 2D Plane 2N Points         
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc091/tasks/arc092_a
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// ## 解説
/// 1. 最もx座標の小さい青い点(bA)を選ぶ
/// 2. bAとマッチングできる赤い点の中で最もy座標の大きい点(rA)を選ぶ
/// → この貪欲法が最適に動作する
fn main() {
    input! {
        N: usize,
        mut A: [(usize, usize); N],
        mut B: [(usize, usize); N],
    }

    // x座標の小さい順にソート
    B.sort_by_key(|v| v.0);

    let mut ans = 0;

    // 貪欲にマッチング
    for &(bx, by) in &B {
        // y座標が最も大きいもののインデックス
        let idx = A.iter()
            .enumerate()
            .filter(|(_, &(ax, ay))| ax < bx && ay < by)
            .max_by_key(|(_, &(ax, ay))| ay)
            .map(|(i, _)| i);

        if let Some(i) = idx {
            ans += 1;
            A.remove(i);
        }
    }

    println!("{}", ans);
}
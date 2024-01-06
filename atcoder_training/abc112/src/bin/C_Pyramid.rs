//               C - Pyramid
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc112/tasks/abc112_c
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// ## 方針
/// - 制約から、全探索できることがわかる
fn main() {
    input! {
        N: usize,
        xyh: [(isize, isize, isize); N],
    }

    // hが0以上のものを選択
    let (mut xt, mut yt, mut ht) = (0, 0, 0);
    for &(x, y, h) in &xyh {
        if h > 0 {
            xt = x;
            yt = y;
            ht = h;
            break;
        }
    }

    // 中心座標を全探索
    for px in 0..=100 {
        for py in 0..=100 {
            // ピラミッドの高さを特定
            let ph = ht + (px - xt).abs() + (py - yt).abs();
            let mut is_ok = true;

            for &(x, y, h) in &xyh {
                is_ok &= h == 0.max( ph - (x - px).abs() - (y - py).abs() );
            }

            if is_ok {
                println!("{} {} {}", px, py, ph);
                return;
            }
        }
    }
}
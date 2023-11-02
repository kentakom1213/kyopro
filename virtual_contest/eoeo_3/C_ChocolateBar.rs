// https://atcoder.jp/contests/abc062/tasks/arc074_a

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// `a > b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmin {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

// solve
fn main() {
    input! {
        H: usize,
        W: usize,
    }

    let mut ans = INF;

    // 縦に3つ
    if W % 3 == 0 {
        ans = 0;
    } else {
        chmin!{
            ans,
            H,  // 1行分の差
        };
    }

    // 横に3つ
    if H % 3 == 0 {
        ans = 0;
    } else {
        chmin!{
            ans,
            W,
        };
    }

    /*
     *   A
     * -----
     * B | C
     */
    for h in 0..H {
        let mut rects = vec![
            h * W,  // A
            (H - h) * (W / 2),  // B
            (H - h) * (W - W / 2)  // C
        ];
        rects.sort();
        chmin!{
            ans,
            rects[2] - rects[0]
        };
    }

    /*
     *   | B
     * A |---
     *   | C
     */
    for w in 0..W {
        let mut rects = vec![
            H * w,  // A
            (H / 2) * (W - w),  // B
            (H - H / 2) * (W - w)  // C
        ];
        rects.sort();
        chmin!{
            ans,
            rects[2] - rects[0]
        };
    }

    println!("{}", ans);
}
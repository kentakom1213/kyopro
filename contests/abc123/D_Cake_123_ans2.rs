//               D - Cake 123
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc123/tasks/abc123_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![warn(clippy::default_numeric_fallback)]

use im_rc::HashSet;
// imports
use itertools::{iproduct, Itertools};
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::{cmp::Reverse, collections::{BTreeSet, BinaryHeap}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

fn main() {
    input! {
        X: usize,
        Y: usize,
        Z: usize,
        K: usize,
        mut A: [usize; X],
        mut B: [usize; Y],
        mut C: [usize; Z],
    }

    // それぞれ逆順ソート
    A.sort_by_key(|&v| Reverse(v));
    B.sort_by_key(|&v| Reverse(v));
    C.sort_by_key(|&v| Reverse(v));

    // MAX-Heapを用意
    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::new();
    pq.push((A[0] + B[0] + C[0], 0, 0, 0));

    for _ in 0..K {
        // 現時点での最大値
        debug!(&pq);
        if let Some((val, i, j, k)) = pq.pop() {
            // 現時点での最大値
            println!("{}", val);
            // 次の候補を追加
            if i + 1 < X && !visited.contains(&(i + 1, j, k)) {

                pq.push((A[i + 1] + B[j] + C[k], i + 1, j, k));
                visited.insert((i + 1, j, k));
            }
            if j + 1 < Y && !visited.contains(&(i, j + 1, k)) {
                pq.push((A[i] + B[j + 1] + C[k], i, j + 1, k));
                visited.insert((i, j + 1, k));
            }
            if k + 1 < Z && !visited.contains(&(i, j, k + 1)) {
                pq.push((A[i] + B[j] + C[k + 1], i, j, k + 1));
                visited.insert((i, j, k + 1));
            }
        }
    }
}

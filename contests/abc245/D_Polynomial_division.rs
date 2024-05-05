//         D - Polynomial division         
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc245/tasks/abc245_d
// ----------------------------------------

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

/// ## 方針
/// - 頑張って割り算の筆算を実装する
/// ```not-rust
///         6   4   2
/// 2, 1 ) 12, 14,  8,  2,
///        12   6
///        ---------------
///             8   8
///             8   4
///             ----------
///                 4   2
///                 4   2
///                 ------
///                     0
/// ```
fn main() {
    input! {
        N: usize,
        M: usize,
        mut A: [isize; N + 1],
        mut C: [isize; N + M + 1],
    }

    A.reverse();
    C.reverse();

    let mut B = vec![];

    for i in 0..=M {
        // 商をたてる
        let d = C[i] / A[0];
        B.push(d);

        // 引き算
        for j in 0..=N {
            C[i + j] -= A[j] * d;
        }
    }

    for &d in B.iter().rev() {
        print!("{} ", d);
    }
    println!();
}
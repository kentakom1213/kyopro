//           D - Range Add Query           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc288/tasks/abc288_d
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

/// ## 解法
/// - https://atcoder.jp/contests/abc288/editorial/5664
/// 
/// ### 不変量
/// modごとの累積和を考える
/// ```not-rust
///     3 -1  1 -2  2  0
/// 0|  3        1←
/// 1|    -1        1←
/// 2|        1        1←
/// ```
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [isize; N],
        Q: usize,
        LR: [(Usize1, usize); Q],
    }

    // modごとに累積和
    let mut cum = vec![vec![0; N+1]; K];
    for j in 0..K {
        for i in 0..N {
            cum[j][i + 1] = cum[j][i] + if i % K == j { A[i] } else { 0 };
        }
    }

    // mod j の区間和
    let get = |j: usize, l: usize, r: usize| -> isize {
        cum[j][r] - cum[j][l]
    };

    // クエリの処理
    for &(l, r) in &LR {
        let mut same = true;
        let val = get(0, l, r);
        for j in 1..K {
            same &= val == get(j, l, r);
        }
        if same {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

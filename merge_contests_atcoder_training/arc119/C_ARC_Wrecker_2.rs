//            B - ARC Wrecker 2            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc119/tasks/arc119_c
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

/// ## 解説
/// - 不変量を考える
/// 
/// ### 1. 判定問題を考える
/// - `A[l:r]`はこの2つの操作で高さを全て0にできるか
///   - Aの偶数番目の要素、奇数番目の要素それぞれの和は常に等しい→不変量
/// - よって、**A[l:r]のうち、偶数番目の要素の和＝奇数番目の要素の和**のとき条件を満たす
/// ```python
/// isOK = sum(A[l:r:2]) == sum(A[l+1:r:2])
/// ```
/// 
/// ### 2. 問題を単純な形に
/// - 以下のような数列Bを考える
///   - iが奇数のとき：`B[i] = A[i]`
///   - iが偶数のとき：`B[i] = -A[i]`
/// - Bの累積和配列を考える
///   - こうすることで、**偶数番目の要素の和 == 奇数番目の要素の和**を`B[r]==B[l-1]`に言い換えられる
/// 
/// ### 3. 計算量を落とす
fn main() {
    input! {
        N: usize,
        A: [isize; N],
    }

    // B[i] := (iが偶数) → -A[i]
    //         (iが奇数) →  A[i]
    // C ← Bの累積和
    let mut C = vec![0; N+1];
    for i in 0..N {
        C[i+1] = C[i] + [-1, 1][i % 2] * A[i];
    }

    // C[i] = C[j] (i != j) となる(i,j)のペアの総数を数える

    let mut mp: BTreeMap<isize, usize> = BTreeMap::new();

    for &c in &C {
        *mp.entry(c).or_insert(0) += 1;
    }

    let mut ans = 0;
    for (_, &v) in &mp {
        ans += v * (v - 1) / 2;
    }

    println!("{}", ans);
}

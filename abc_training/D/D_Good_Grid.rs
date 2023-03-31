//              D - Good Grid
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc099/tasks/abc099_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
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
/// - 色Cが30色しかないから、多少無茶な操作もできる
/// - (塗り替える前, 塗り替える後)の色の組合せを全探索
fn main() {
    input! {
        N: usize,
        C: usize,
        D: [[usize; C]; C],
        F: [[Usize1; N]; N],
    }

    // 全てのますの色を`(i+j)%3`ごとに数える
    let mut cnt = vec![vec![0; 30]; 3];
    for i in 0..N {
        for j in 0..N {
            cnt[(i + j) % 3][F[i][j]] += 1;
        }
    }

    // 塗り替えた後の色の組合せを全探索
    let mut ans = INF;

    for color in (0..C).permutations(3) {
        // 塗り替えるのにかかるコストを計算
        let mut cost = 0;
        for (i, &after) in color.iter().enumerate() {
            for before in 0..C {
                cost += D[before][after] * cnt[i][before];
            }
        }
        if cost < ans {
            ans = cost;
        }
    }

    println!("{}", ans);
}

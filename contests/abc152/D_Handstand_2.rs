//             D - Handstand 2             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc152/tasks/abc152_d
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

// solve
fn main() {
    input! {N: usize}

    // cnt[i][j] := Nまでの自然数でiで始まってjで終わる数の合計
    let mut cnt = vec![vec![0; 10]; 10];

    for i in 1..=N {
        let tail = i % 10;
        let head = {
            let mut x = i;
            while x >= 10 {
                x /= 10;
            }
            x
        };
        cnt[head][tail] += 1;
    }

    // 組合せを求める
    let mut ans = 0;
    for i in 0..10 {
        for j in 0..10 {
            ans += cnt[i][j] * cnt[j][i];
        }
    }

    println!("{}", ans);
}

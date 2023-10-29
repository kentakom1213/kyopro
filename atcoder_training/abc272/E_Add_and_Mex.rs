//             E - Add and Mex
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc272/tasks/abc272_e
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: isize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// solve
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [isize; N],
    }

    let mut arrays = vec![vec![]; M+1];

    // 計算結果を生成
    // 計算量を調和級数に帰着
    for (i, &a) in A.iter().enumerate() {
        // a + (i+1) * j >= 0 となる最小のj
        let start: isize = 1.max( - a / (1 + i as isize) - 1 );
        for j in start..=(M as isize) {
            let val = a + (1 + i as isize) * j as isize;
            arrays[j as usize].push(val);
            if val > N as isize { break; }
        }
    }
    
    // 配列に含まれない最小の非負整数を求める
    for i in 1..=M {
        let ref arr = arrays[i];
        let mut exists = vec![false; arr.len()+1];
        for &v in arr {
            if 0 <= v && v <= arr.len() as isize {
                exists[v as usize] = true;
            }
        }

        let mut ans = 0;
        while exists[ans] {
            ans += 1;
        }
        println!("{}", ans);
    }
}

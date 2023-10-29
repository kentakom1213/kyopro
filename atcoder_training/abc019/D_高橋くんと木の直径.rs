//              D - 高橋くんと木の直径
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc019/tasks/abc019_4
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
    source::line::LineSource,
};
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

/// `a < b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmax {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a < $b {
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
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// solve
fn main() {
    let stdin = std::io::stdin();
    let mut source = LineSource::new(stdin.lock());
    input! {
        from &mut source,
        N: usize,
    }

    // 頂点1から頂点iまでの距離とその最大値
    let (mut dmax, mut imax) = (0, 0);
    for i in 2..=N {
        println!("? 1 {}", i);
        input! {
            from &mut source,
            d: usize,
        }
        if chmax!(dmax, d) {
            imax = i;
        }
    }

    // 頂点1から最も遠い頂点からの距離
    let mut ans = 0;
    for j in 1..=N {
        println!("? {} {}", imax, j);
        input! {
            from &mut source,
            d: usize,
        }
        chmax! {
            ans,
            d,
        };
    }
    println!("! {}", ans);
}

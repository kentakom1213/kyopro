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

/// `a < b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmax {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a <= $b {
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
        N: usize,
        SS: [String; N],
        M: usize,
        TT: [String; M],
    }

    let mut map: BTreeMap<&str, isize> = BTreeMap::new();

    for s in &SS {
        *map.entry(s).or_insert(0) += 1;
    }

    for t in &TT {
        *map.entry(t).or_insert(0) -= 1;
    }

    // valueが最大の値を答える
    let mut ans = 0;

    for (&k, &v) in &map {
        chmax!{
            ans,
            v
        };
    }

    println!("{}", ans);
}
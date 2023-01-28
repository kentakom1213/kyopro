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
    input! {
        S: Chars,
        T: Chars,
    }

    let sl = S.len();
    let tl = T.len();

    // 尺取り法的に考える
    // 初期化
    let mut dist = 0;  // 異なる文字の数

    for (&t, &s) in T.iter().zip(S[sl-tl..].iter()) {
        if t != '?' && s != '?' && t != s {
            dist += 1;
        }
    }
    if dist == 0 {
        println!("Yes");
    } else {
        println!("No");
    }

    // 尺取り
    for i in 0..tl {
        // T[i] を S[sl-tl-i] -> S[i] に移動させる
        let s1 = S[sl-tl+i];
        let s2 = S[i];
        let t = T[i];

        if t != '?' && s1 != '?' && s1 != t {
            dist -= 1;
        }
        if t != '?' && s2 != '?' && s2 != t {
            dist += 1;
        }
        if dist == 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
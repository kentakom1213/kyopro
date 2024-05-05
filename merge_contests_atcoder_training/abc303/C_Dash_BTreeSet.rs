//                 C - Dash                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc303/tasks/abc303_c
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::BTreeSet;

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

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

// main
fn main() {
    input! {
        N: usize,
        M: usize,
        H: isize,
        K: isize,
        S: String,
        items: [(isize, isize); M],
    }

    // items
    let mut set = BTreeSet::new();
    for &(x, y) in &items {
        set.insert((x, y));
    }

    // 移動
    let (mut x, mut y) = (0, 0);
    let mut hp = H;
    for c in S.chars() {
        match c {
            'R' => x += 1,
            'L' => x -= 1,
            'U' => y += 1,
            'D' => y -= 1,
            _ => (),
        }
        hp -= 1;
        if hp < 0 {
            println!("No");
            return;
        }
        else if hp < K && set.remove(&(x, y)) {
            hp = K;
        }
    }

    println!("Yes");
}

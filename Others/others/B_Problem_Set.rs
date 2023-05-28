//             B - Problem Set             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/code-festival-2017-qualb/tasks/code_festival_2017_qualb_b?lang=ja
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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
        mut D: [usize; N],
        M: usize,
        mut T: [usize; M],
    }

    // 配列をソート
    D.sort();
    T.sort();

    // 先頭からマージ
    let (mut i, mut j) = (0, 0);
    
    while i < N && j < M {
        if D[i] == T[j] {
            i += 1;
            j += 1;
        }
        else if D[i] < T[j] {
            i += 1;
        }
        else if D[i] > T[j] {
            break;
        }
    }

    // Mが全て消費されていればOK
    if j == M {
        println!("YES");
    } else {
        println!("NO");
    }
}

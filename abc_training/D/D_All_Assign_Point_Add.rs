//         D - All Assign Point Add        
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc278/tasks/abc278_d
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
type Graph = Vec<Vec<usize>>;

// main
#[fastout]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
        Q: usize,
    }

    // 遅延値が適用されているかどうか
    let mut latest = 0;
    let mut delay_val = 0;
    let mut version = vec![0; N];

    for _ in 0..Q {
        input!{q: usize}
        if q == 1 {
            input!{x: usize}
            latest += 1;
            delay_val = x;
        }
        else if q == 2 {
            input!{i: Usize1, x: usize}
            if version[i] < latest {
                // 遅延値を適用
                version[i] = latest;
                A[i] = delay_val;
            }
            A[i] += x;
            version[i] = latest;
        }
        else {
            input!{i: Usize1}
            if version[i] < latest {
                // 遅延値を適用
                version[i] = latest;
                A[i] = delay_val;
            }
            println!("{}", A[i]);
        }
        debug!(&A, &delay_val);
    }
}

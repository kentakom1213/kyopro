//               A - Make 10               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc126/tasks/arc126_a
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

fn debug_2d<T: std::fmt::Debug>(array: &Vec<Vec<T>>) {
    #[cfg(debug_assertions)]
    for row in array {
        println!("{:?}", row);
    }
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// main
fn main() {
    input! {
        T: usize,
    }

    for _ in 0..T {
        solve();
    }
}

fn solve() {
    input! {
        n2: usize,
        n3: usize,
        n4: usize,
    }

    let n6 = n3 / 2; // ながさ6の棒が n3/2 本あると考える

    // 棒の長さを2でわり、1,2,3の棒が存在すると考える
    let mut n1 = n2;
    let mut n2 = n4;
    let mut n3 = n6;
    debug!(n1, n2, n3);

    // 答え
    let mut ans = 0;

    // 1. 2 + 3
    let tmp = n2.min(n3);
    ans += tmp;
    n2 -= tmp;
    n3 -= tmp;

    // 2. 3 + 1 + 1
    let tmp = n3.min(n1 / 2);
    ans += tmp;
    n3 -= tmp;
    n1 -= tmp * 2;

    // 3. 2 + 2 + 1
    let tmp = n1.min(n2 / 2);
    ans += tmp;
    n1 -= tmp;
    n2 -= tmp * 2;

    // 4. 2 + 1 + 1 + 1
    let tmp = n2.min(n1 / 3);
    ans += tmp;
    n1 -= tmp * 3;
    n2 -= tmp;

    // 5. 1 + 1 + 1 + 1 + 1
    let tmp = n1 / 5;
    ans += tmp;
    
    println!("{}", ans);
}

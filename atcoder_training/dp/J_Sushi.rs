//                J - Sushi
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/dp/tasks/dp_j
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
const INF: f64 = -1.0;
const NEG1: usize = 1_usize.wrapping_neg();

fn div_f64(a: usize, b: usize) -> f64 {
    (a as f64) / (b as f64)
}

// main
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    // それぞれの個数をカウント
    let mut cnt = [0_usize; 4];
    for &a in &A {
        cnt[a] += 1;
    }
    let [_, C1, C2, C3] = cnt;

    // メモ
    let mut mem = vec![vec![vec![INF; N + 1]; N + 1]; N + 1];

    let ans = rec(N, C1, C2, C3, &mut mem);

    println!("{}", ans);
}

fn rec(N: usize, c1: usize, c2: usize, c3: usize, mem: &mut Vec<Vec<Vec<f64>>>) -> f64 {
    if mem[c1][c2][c3] != INF {
        return mem[c1][c2][c3];
    }

    debug!(c1, c2, c3);

    if c1 == 0 && c2 == 0 && c3 == 0 {
        return 0.0;
    }

    let mut res = 1.0;

    // を選ぶとき
    if c1 >= 1 {
        res += rec(N, c1 - 1, c2, c3, mem) * div_f64(c1, N);
    }

    // を選ぶとき
    if c2 >= 1 {
        res += rec(N, c1 + 1, c2 - 1, c3, mem) * div_f64(c2, N);
    }

    // を選ぶとき
    if c3 >= 1 {
        res += rec(N, c1, c2 + 1, c3 - 1, mem) * div_f64(c3, N);
    }

    res *= div_f64(N, c1 + c2 + c3);

    mem[c1][c2][c3] = res;
    res
}

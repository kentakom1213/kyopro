#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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

macro_rules! debug2D {
    ( $array:expr ) => {{
        #![cfg(debug_assertions)]
        eprintln!("{}: ", stringify!($array));
        for row in &$array {
            eprintln!("{:?}", row);
        }
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        K: usize,
        mut A: [Usize1; K],
    }

    // Kが偶数のとき
    if K % 2 == 0 {
        let mut ans = 0;
        for i in 0..K / 2 {
            ans += A[2 * i + 1] - A[2 * i];
        }
        println!("{ans}");
        return;
    }

    // ペアが揃っているものは無視してok
    // 隣同士の差分
    // 左から
    let mut left = vec![0];
    for i in 0..K / 2 {
        left.push(A[2 * i + 1] - A[2 * i]);
    }
    // 右から
    A.reverse();
    let mut right = vec![];
    for i in 0..K / 2 {
        right.push(A[2 * i] - A[2 * i + 1]);
    }

    debug!(left, right);

    // それぞれ累積和
    for i in 1..K / 2 + 1 {
        left[i] += left[i - 1];
    }
    for i in 1..K / 2 {
        right[i] += right[i - 1];
    }
    right.reverse();
    right.push(0);
    debug!(left, right);

    // スライドさせて最小値を求める
    let mut ans = INF;

    for i in 0..=K / 2 {
        ans = ans.min(left[i] + right[i]);
    }

    println!("{ans}");
}

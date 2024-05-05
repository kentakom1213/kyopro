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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        P: [Chars; N],
        queries: [(usize, usize, usize, usize); Q]
    }

    // つなげて考える
    let mut arr = vec![vec![0_usize; 2 * N]; 2 * N];

    for r in 0..N {
        for c in 0..N {
            if P[r][c] == 'B' {
                arr[r][c] = 1;
                arr[r][N + c] = 1;
                arr[N + r][c] = 1;
                arr[N + r][N + c] = 1;
            }
        }
    }

    // 2次元累積和
    let S = acc2D(&arr);

    // クエリ処理
    for &(mut rs, mut cs, mut re, mut ce) in &queries {
        let hrep = (re - rs) / N;
        let wrep = (ce - cs) / N;

        rs %= N;
        cs %= N;
        re %= N;
        ce %= N;
        if rs > re {
            re += N;
        }
        if cs > ce {
            ce += N;
        }

        debug!(hrep, wrep);

        // 中央
        let center = S(0, N, 0, N) * hrep * wrep;

        // 上下
        let tb = S(rs, re + 1, 0, N) * wrep;

        // 左右
        let lr = S(0, N, cs, ce + 1) * hrep;

        // 端部分
        let edge = S(rs, re + 1, cs, ce + 1);

        let res = center + tb + lr + edge;

        println!("{}", res);
    }
}

use num_traits::Num;
/// ## acc2D
/// - 2次元累積和を取る
/// ### 戻り値
/// - `|r_start, r_end, c_start, c_end|: (usize, usize, usize, usize) -> T`
pub fn acc2D<T: Num + Copy>(array: &Vec<Vec<T>>) -> impl Fn(usize, usize, usize, usize) -> T {
    let (H, W) = (array.len(), array[0].len());
    let mut S = vec![vec![T::zero(); W + 1]; H + 1];
    for i in 0..H {
        for j in 0..W {
            S[i + 1][j + 1] = S[i][j + 1] + S[i + 1][j] - S[i][j] + array[i][j];
        }
    }
    move |r_start: usize, r_end: usize, c_start: usize, c_end: usize| -> T {
        S[r_end][c_end] + S[r_start][c_start] - S[r_end][c_start] - S[r_start][c_end]
    }
}

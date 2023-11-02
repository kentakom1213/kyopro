// https://atcoder.jp/contests/abc278/tasks/abc278_e

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

pub fn acc2D(array: &Vec<Vec<isize>>) -> impl Fn(usize, usize, usize, usize) -> isize {
    let (H, W) = (array.len(), array[0].len());
    let mut S = vec![vec![0; W + 1]; H + 1];
    for i in 0..H {
        for j in 0..W {
            S[i + 1][j + 1] = S[i][j + 1] + S[i + 1][j] - S[i][j] + array[i][j];
        }
    }
    move |r_start: usize, r_end: usize, c_start: usize, c_end: usize| -> isize {
        S[r_end][c_end]
            - S[r_end][c_start]
            - S[r_start][c_end]
            + S[r_start][c_start]
    }
}

// main
fn main() {
    input! {
        H: usize,
        W: usize,
        N: usize,
        h: usize,
        w: usize,
        A: [[Usize1; W]; H],
    }

    // 色に分解
    let mut colors = vec![vec![vec![0; W]; H]; N];
    for i in 0..H {
        for j in 0..W {
            let c = A[i][j];
            colors[c][i][j] = 1;
        }
    }

    // 色ごとに二次元累積和をとる
    let mut acc = vec![];
    for arr in &colors {
        acc.push(acc2D(arr));
    }

    // 全ての色の要素数を保存
    let color_all = (0..N).map(|i| acc[i](0, H, 0, W)).collect_vec();

    // クエリ処理
    for i in 0..H - h + 1 {
        for j in 0..W - w + 1 {
            let mut kinds = 0;
            for c in 0..N {
                let cnt = acc[c](i, i + h, j, j + w);
                if color_all[c] > cnt {
                    kinds += 1;
                }
            }
            print!("{} ", kinds);
        }
        println!();
    }
}

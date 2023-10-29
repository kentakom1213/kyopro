//         E - Defect-free Squares
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc311/tasks/abc311_e
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

use num_traits::Num;

pub fn acc2D<T: Num + Copy>(array: &Vec<Vec<T>>) -> impl Fn(usize, usize, usize, usize) -> T {
    let (H, W) = (array.len(), array[0].len());
    let mut S = vec![vec![T::zero(); W + 1]; H + 1];
    for i in 0..H {
        for j in 0..W {
            S[i + 1][j + 1] = S[i][j + 1] + S[i + 1][j] - S[i][j] + array[i][j];
        }
    }
    move |r_start: usize, r_end: usize, c_start: usize, c_end: usize| -> T {
        S[r_end][c_end] - S[r_end][c_start] - S[r_start][c_end] + S[r_start][c_start]
    }
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// main
fn main() {
    input! {
        H: usize,
        W: usize,
        N: usize,
        holes: [(Usize1, Usize1); N]
    }

    // グリッド
    let mut grid = vec![vec![0; W]; H];
    for &(a, b) in &holes {
        grid[a as usize][b as usize] += 1;
    }

    if cfg!(debug_assertions) {
        for r in &grid {
            println!("{:?}", r);
        }
    }

    // 二次元累積和
    let acc = acc2D(&grid);

    // 左上の座標が(r,c)である穴のない長方形の一辺の長さの最大値
    let maxSize = |r: usize, c: usize| -> usize {
        let mut ok = 1_usize.wrapping_neg();
        let mut ng = (H - r).min(W - c);
        while ng.wrapping_sub(ok) > 1 {
            let mid = ok.wrapping_add(ng) / 2;
            if acc(r, r.wrapping_add(mid + 1), c, c.wrapping_add(mid + 1)) == 0 {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok.wrapping_add(1)
    };

    // 全てのますについて、左と下に二分探索
    let mut ans: usize = 0;

    for r in 0..H {
        for c in 0..W {
            let size = maxSize(r, c);
            debug!(r, c, size);
            ans += size;
        }
    }

    println!("{}", ans);
}

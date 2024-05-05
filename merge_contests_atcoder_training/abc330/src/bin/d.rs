// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}, fastout};

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

fn main() {
    input! {
        N: usize,
        S: [Chars; N]
    }

    // マスの個数の累積和
    let mut rows = vec![vec![0; N + 1]; N];

    for r in 0..N {
        for c in 0..N {
            if S[r][c] == 'o' {
                rows[r][c + 1] = rows[r][c] + 1;
            } else {
                rows[r][c + 1] = rows[r][c];
            }
        }
    }

    let mut cols = vec![vec![0; N]; N + 1];

    for c in 0..N {
        for r in 0..N {
            if S[r][c] == 'o' {
                cols[r + 1][c] = cols[r][c] + 1;
            } else {
                cols[r + 1][c] = cols[r][c];
            }
        }
    }

    debug!(rows);
    debug!(cols);

    // マスを全探索
    let mut ans = 0_usize;

    for r in 0..N {
        for c in 0..N {
            let left = rows[r][c];
            let right = rows[r][N] - rows[r][c + 1];
            let up = cols[r][c];
            let down = cols[N][c] - cols[r + 1][c];

            let row = left + right;
            let col = up + down;

            // 組合せの計算
            let tmp = row * col;

            if S[r][c] == 'o' {
                ans += tmp;
            }

            debug!(r, c, left, right, up, down, row, col, tmp);
        }
    }

    println!("{}", ans);
}

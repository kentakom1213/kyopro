// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::{iproduct, Itertools};
use proconio::{
    input,
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

fn main() {
    input! {
        H: usize,
        W: usize,
        N: usize,
        M: usize,
        AB: [(Usize1, Usize1); N],
        CD: [(Usize1, Usize1); M],
    }

    let mut rows = vec![vec![0; W]; H];

    // 電球をセット
    for &(a, b) in &AB {
        rows[a][b] += 1;
    }

    // ブロックをセット
    for &(c, d) in &CD {
        rows[c][d] -= 1;
    }

    // 累積和をとる
    for r in 0..H {
        // 左→右
        for c in 1..W {
            if rows[r][c - 1] == 1 && rows[r][c] == 0 {
                rows[r][c] = 1;
            }
        }
        // 右→左
        for c in (0..W - 1).rev() {
            if rows[r][c + 1] == 1 && rows[r][c] == 0 {
                rows[r][c] = 1;
            }
        }
    }

    let mut cols = vec![vec![0; W]; H];

    // 電球をセット
    for &(a, b) in &AB {
        cols[a][b] += 1;
    }

    // ブロックをセット
    for &(c, d) in &CD {
        cols[c][d] -= 1;
    }

    // 累積和をとる
    for c in 0..W {
        // 左→右
        for r in 1..H {
            if cols[r - 1][c] == 1 && cols[r][c] == 0 {
                cols[r][c] = 1;
            }
        }
        // 右→左
        for r in (0..H - 1).rev() {
            if cols[r + 1][c] == 1 && cols[r][c] == 0 {
                cols[r][c] = 1;
            }
        }
    }

    if cfg!(debug_assertions) {
        eprintln!("rows");
        for row in &rows {
            eprintln!("{:?}", row);
        }
    }

    if cfg!(debug_assertions) {
        eprintln!("cols");
        for row in &cols {
            eprintln!("{:?}", row);
        }
    }

    // カウント
    let ans = iproduct!(0..H, 0..W)
        .filter(|&(r, c)| rows[r][c] == 1 || cols[r][c] == 1)
        .count();

    println!("{}", ans);
}

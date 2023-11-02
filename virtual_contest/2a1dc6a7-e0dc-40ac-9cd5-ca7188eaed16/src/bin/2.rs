// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}};

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
        mut A: [[usize; W]; H]
    }

    let mut cmd = vec![];

    // 右に寄せる
    for r in 0..H {
        for c in 0..W - 1 {
            if A[r][c] % 2 == 1 {
                cmd.push((r, c, r, c + 1));
                A[r][c] -= 1;
                A[r][c + 1] += 1;
            }
        }
    }

    if cfg!(debug_assertions) {
        for r in &A {
            eprintln!("{:?}", r);
        }
    }
    
    // 下に寄せる
    for r in 0..H - 1 {
        if A[r][W - 1] % 2 == 1 {
            cmd.push((r, W - 1, r + 1, W - 1));
            A[r][W - 1] -= 1;
            A[r + 1][W - 1] += 1;
        }
    }

    if cfg!(debug_assertions) {
        for r in &A {
            eprintln!("{:?}", r);
        }
    }

    // 出力
    println!("{}", cmd.len());
    for &(a, b, c, d) in &cmd {
        println!("{} {} {} {}", a + 1, b + 1, c + 1, d + 1);
    }
}

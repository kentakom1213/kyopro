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

// main
fn main() {
    input! {
        H: usize,
        W: usize,
        N: usize,
        holes: [(Usize1, Usize1); N],
    }

    // 穴を開ける
    let mut field = vec![vec![false; W]; H];
    for &(a, b) in &holes {
        field[a][b] = true;
    }

    // last[r][c] := 最も近くにある穴のマンハッタン距離
    let mut last = vec![vec![0; W + 1]; H + 1];
    let mut ans: usize = 0;

    for i in 0..H {
        for j in 0..W {
            if field[i][j] {
                last[i + 1][j + 1] = 0;
            } else {
                last[i + 1][j + 1] = last[i][j].min(last[i][j + 1]).min(last[i + 1][j]) + 1;
            }
            // 加算
            ans += last[i + 1][j + 1];
        }
    }

    // if cfg!(debug_assertions) {
    //     for r in &last {
    //         println!("{:?}", r);
    //     }
    // }

    println!("{}", ans);
}

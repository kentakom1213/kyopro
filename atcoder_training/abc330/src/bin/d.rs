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

fn main() {
    input! {
        N: usize,
        S: [Chars; N],
    }

    let (rows, cols) =
        iproduct!(0..N, 0..N).fold((vec![0; N], vec![0; N]), |(mut rows, mut cols), (r, c)| {
            if S[r][c] == 'o' {
                rows[r] += 1;
                cols[c] += 1;
            }
            (rows, cols)
        });

    // 全探索
    let ans = iproduct!(0..N, 0..N)
        .map(|(r, c)| {
            if S[r][c] == 'o' {
                (rows[r] - 1) * (cols[c] - 1)
            } else {
                0
            }
        })
        .sum::<usize>();

    println!("{}", ans);
}

const INF: usize = 1001001001001001001;

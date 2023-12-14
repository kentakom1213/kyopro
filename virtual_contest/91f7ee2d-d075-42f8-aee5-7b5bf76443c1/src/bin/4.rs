// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
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
        N: usize,
        A: [usize; N]
    }

    // // 愚直解
    // let mut ans = 0;
    // for i in 0..N {
    //     for j in i + 1..N {
    //         if A[i] != A[j] {
    //             ans += (i + 1).min(N - j);
    //         }
    //     }
    // }
    // println!("{ans}");

    // すべてのペアの個数
    let total = (1..=N).map(|i| (N + 1 - i) * (i / 2)).sum::<usize>();

    // bucket
    let indexes = (0..N).fold(vec![vec![]; N + 1], |mut idx, i| {
        idx[A[i]].push(i + 1);
        idx
    });

    // いいペアの個数
    let mut good = 0;

    for P in indexes {
        let mut l = 0;
        let mut r = P.len().saturating_sub(1);
        while l < r {
            if P[l] < N + 1 - P[r] {
                good += (r - l) * P[l];
                l += 1;
            } else {
                good += (r - l) * (N + 1 - P[r]);
                r -= 1;
            }
        }
    }

    let ans = total - good;

    println!("{ans}");
}

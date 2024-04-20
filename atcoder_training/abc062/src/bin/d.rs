#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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

use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

fn main() {
    input! {
        N: usize,
        A: [isize; N * 3]
    }

    // D: 削除する値の集合
    // X: A\D の先頭N個
    // Y: A\D の末尾N個
    // 定義から、
    // A = X + Y + D
    //
    // ここで、
    //     max. sum(X) - sum(Y)
    // ==> max. (sum(X) - sum(Y)) + sum(A)
    // ==> max. 2*sum(X) + sum(D)
    //
    // 先頭N個と、削除するN個のみを調べれば良い

    // 全体の総和
    let Sa: isize = A.iter().sum();

    // 先頭2N個のうち大きいものN個の総和
    let Sx: isize = A[..2 * N].iter().sorted().skip(N).sum();

    // 末尾N個の総和
    let Sy: isize = A[2 * N..].iter().sum();

    let ans = Sx - Sy;

    println!("{ans}");
}

const INF: usize = 1001001001001001001;

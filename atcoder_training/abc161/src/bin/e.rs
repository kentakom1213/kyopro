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

use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

fn main() {
    input! {
        N: usize,
        K: usize,
        C: usize,
        S: Chars
    }

    // 前から貪欲に決めていく
    let mut L = vec![0; N];

    let mut i = 0;
    let mut cnt = 0;
    while cnt < K {
        if S[i] == 'o' {
            cnt += 1;
            L[i] = cnt;
            i += C;
        }
        i += 1;
    }

    debug!(L);

    // 後ろから貪欲に決めていく
    let mut R = vec![0; N];

    let mut i = N - 1;
    let mut cnt = K;
    while cnt > 0 {
        debug!(i, cnt, R);
        if S[i] == 'o' {
            R[i] = cnt;
            cnt -= 1;
            if cnt == 0 {
                break;
            }
            i -= C;
        }
        i -= 1;
    }

    debug!(R);

    // 答えを求める
    for (i, (&a, &b)) in L.iter().zip(&R).enumerate() {
        if a > 0 && a == b {
            println!("{}", i + 1);
        }
    }
}

const INF: usize = 1001001001001001001;

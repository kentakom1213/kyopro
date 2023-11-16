// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use num_integer::gcd;
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

/// - 素因数分解し、`(素因数,指数)`のペアを返す
pub fn factorize(mut n: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for i in 2.. {
        if i * i > n {
            break;
        }
        let mut cnt = 0;
        while n % i == 0 {
            n /= i;
            cnt += 1;
        }
        if cnt >= 1 {
            res.push((i, cnt));
        }
    }
    if n > 1 {
        res.push((n, 1));
    }
    res
}

fn main() {
    input! {
        A: usize,
        B: usize,
    }

    // 最大公約数を求める
    let G = gcd(A, B);

    // 素因数分解
    let factors = factorize(G);

    debug!(factors);

    println!("{}", factors.len() + 1);
}

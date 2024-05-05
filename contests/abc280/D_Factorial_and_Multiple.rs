//        D - Factorial and Multiple       
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc280/tasks/abc280_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

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
        K: usize,
    }

    // Kを素因数分解
    let fac = factorize(K);

    debug!(&fac);

    // 判定関数
    // - x!がKの倍数であるか
    let isok = |x: usize| -> bool {
        for &(p, a) in &fac {
            let mut cnt = 0_usize;
            let mut rem = x;
            while rem > 0 {
                cnt += rem / p;
                rem /= p;
            }
            // 割り切れない
            if cnt < a {
                return false;
            }
        }
        true
    };

    // 二分探索
    let mut ng = 0;
    let mut ok = INF;

    while ok - ng > 1 {
        let mid = (ng + ok) / 2;
        if isok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}

fn factorize(mut n: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for i in 2.. {
        if i*i > n {
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

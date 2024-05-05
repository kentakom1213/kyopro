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
        A: u128,
        B: u128
    }

    // 禁止されていない数を数え上げ

    // 10進数xを4,9を含まない10進数(8進数)に変換
    let to_dec_without_4_9 = |mut x: u128| -> u128 {
        let mut res = 0;
        let mut dd = 1;
        while x > 0 {
            let d = x % 8;
            if d < 4 {
                res += d * dd;
            } else {
                res += (d + 1) * dd;
            }
            dd *= 10;
            x /= 8;
        }
        res
    };

    // Aを超えない最大の4,9抜き10進数の番号
    let l = {
        let mut ok = 0;
        let mut ng = INF;
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            let dec = to_dec_without_4_9(mid);
            if dec < A {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    };
    // B以下で最大の4,9抜き10進数の番号
    let r = {
        let mut ok = 0;
        let mut ng = INF;
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            let dec = to_dec_without_4_9(mid);
            if dec <= B {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    };

    let without_4_9 = r - l;
    let prohibited = (B - A + 1) - without_4_9;

    println!("{prohibited}");
}

const INF: u128 = 1001001001001001001001;

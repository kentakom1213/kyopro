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

/// `chmin!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最小のものを、`x1`に代入する
/// - 代入があったとき、`true`を返す
#[macro_export]
macro_rules! chmin {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
    ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
        chmin! {
            $a,
            ($b).min($c)
            $(,$other)*
        }
    }};
}

use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};
use rustc_hash::FxHashMap;

fn main() {
    input! {
        N: usize,
        Ma: isize,
        Mb: isize,
        ABC: [(isize, isize, isize); N]
    }

    let left = &ABC[..N / 2];
    let right = &ABC[N / 2..];

    debug!(left);
    debug!(right);

    let mut ans = INF;

    // 前半の情報を保存
    let mut map = FxHashMap::default();

    for i in 1..1 << left.len() {
        let mut sum_a = 0;
        let mut sum_b = 0;
        let mut sum_c = 0;
        for j in 0..left.len() {
            if i >> j & 1 == 1 {
                let (a, b, c) = left[j];
                sum_a += a;
                sum_b += b;
                sum_c += c;
            }
        }
        let key = sum_a * Mb - sum_b * Ma;
        if key == 0 {
            chmin! {
                ans,
                sum_c
            };
        }
        chmin! {
            *map.entry(key).or_insert(sum_c),
            sum_c
        };
    }

    debug!(map);

    // 後半を全探索
    for i in 1..1 << right.len() {
        let mut sum_a = 0;
        let mut sum_b = 0;
        let mut sum_c = 0;
        for j in 0..right.len() {
            if i >> j & 1 == 1 {
                let (a, b, c) = right[j];
                sum_a += a;
                sum_b += b;
                sum_c += c;
            }
        }
        let key = sum_a * Mb - sum_b * Ma;
        if key == 0 {
            chmin! {
                ans,
                sum_c
            };
        }
        if let Some(&l_sum) = map.get(&-key) {
            chmin! {
                ans,
                l_sum + sum_c
            };
        }
    }

    if ans == INF {
        println!("-1");
    } else {
        println!("{ans}");
    }
}

const INF: isize = 1001001001001001001;

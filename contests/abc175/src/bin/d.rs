// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::f32::MIN;

use im_rc::HashMap;
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

/// `chmax!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最大のものを、`x1`に代入する
/// - 代入があったとき、`true`を返す
#[macro_export]
macro_rules! chmax {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
    ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
        chmax! {
            $a,
            ($b).max($c)
            $(,$other)*
        }
    }}
}

const INF: usize = 1001001001001001001;
const MINF: isize = -(INF as isize);

fn main() {
    input! {
        N: usize,
        K: usize,
        P: [Usize1; N],
        C: [isize; N],
    }

    let mut ans = MINF;

    // 始点を全探索
    for i in 0..N {
        // 1. ループ全体の合計を調べる
        let mut v = i;  // スタート
        let mut cycle_sum = 0;
        let mut cycle_cnt = 0;

        loop {
            cycle_cnt += 1;
            cycle_sum += C[v];
            v = P[v];
            if v == i {
                break;
            }
        }

        // 2. 終点を決めてループする
        let mut path = 0;
        let mut cnt = 0;

        loop {
            cnt += 1;
            path += C[v];

            if cnt > K {
                break;
            }

            let num = (K - cnt) / cycle_cnt;
            let score = path + 0.max(cycle_sum) * num as isize;
            chmax! {
                ans,
                score
            };

            v = P[v];
            if v == i {
                break;
            }
        }
    }

    println!("{}", ans);
}

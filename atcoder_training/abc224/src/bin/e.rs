// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::cmp::Reverse;

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};
use rustc_hash::FxHashMap;

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

fn main() {
    input! {
        H: usize,
        W: usize,
        N: usize,
        rca: [(usize, usize, usize); N]
    }

    let mut rmax = FxHashMap::<usize, usize>::default();
    let mut cmax = FxHashMap::<usize, usize>::default();
    let mut dp = FxHashMap::<(usize, usize), usize>::default();

    // aの降順に処理
    for (a, itr) in &rca
        .iter()
        .cloned()
        .sorted_by_key(|x| Reverse(x.2))
        .group_by(|x| x.2)
    {
        let vec = itr.collect_vec();

        for &(r, c, a) in &vec {
            let mut tmp = 0;

            if let Some(&max) = rmax.get(&r) {
                chmax! { tmp, max + 1 };
            }

            if let Some(&max) = cmax.get(&c) {
                chmax! { tmp, max + 1 };
            }

            // dpの更新
            chmax! {
                *dp.entry((r, c)).or_insert(0),
                tmp
            };
        }
        // 更新
        for &(r, c, a) in &vec {
            let max = dp[&(r, c)];

            chmax! {
                *rmax.entry(r).or_insert(0),
                max
            };

            chmax! {
                *cmax.entry(c).or_insert(0),
                max
            };
        }
    }

    println!("{}", rca.iter().map(|&(r, c, _)| dp[&(r, c)]).join("\n"));
}

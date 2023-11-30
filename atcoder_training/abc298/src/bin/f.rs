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
use rustc_hash::{FxHashMap, FxHashSet};

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
        N: usize,
        RCX: [(usize, usize, usize); N]
    }

    let (cells, rows, cols) = RCX.iter().fold(
        (
            FxHashMap::default(),
            FxHashMap::default(),
            FxHashMap::default(),
        ),
        |(mut cells, mut rows, mut cols), &(r, c, x)| {
            cells.insert((r, c), x);
            *rows.entry(r).or_insert(0) += x;
            *cols.entry(c).or_insert(0) += x;
            (cells, rows, cols)
        },
    );

    debug!(cells);
    debug!(rows);
    debug!(cols);

    let rows = rows
        .into_iter()
        .sorted_by_key(|&(k, v)| (Reverse(v), k))
        .collect_vec();

    let cols = cols
        .into_iter()
        .sorted_by_key(|&(k, v)| (Reverse(v), k))
        .collect_vec();

    debug!(rows);
    debug!(cols);

    // 行を全探索
    let mut ans = rows[0].1.max(cols[0].1);

    for &(row, rmax) in &rows {
        for &(col, cmax) in &cols {
            if let Some(&x) = cells.get(&(row, col)) {
                chmax! {
                    ans,
                    rmax + cmax - x
                };
            } else {
                chmax! {
                    ans,
                    rmax + cmax
                };
                break;
            }
        }
    }

    println!("{}", ans);
}

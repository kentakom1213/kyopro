// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::{BTreeMap, BTreeSet};

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1}, fastout,
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

const INF: usize = 1001001001001001001;

#[fastout]
fn main() {
    input! {
        H: isize,
        W: isize,
        rs: isize,
        cs: isize,
        N: usize,
        blocks: [(isize, isize); N],
        Q: usize,
    }

    let mut rows = BTreeMap::new();
    let mut cols = BTreeMap::new();

    // ブロックの直前、直後を追加
    for &(r, c) in &blocks {
        rows.entry(r).or_insert_with(|| BTreeSet::new()).insert(c);
        cols.entry(c).or_insert_with(|| BTreeSet::new()).insert(r);
    }

    debug!(rows);
    debug!(cols);

    let (mut r, mut c) = (rs, cs);

    // クエリ処理
    for _ in 0..Q {
        input! {
            d: char,
            l: isize,
        }
        match d {
            'L' => {
                c = migrate(r, c, -l, W, &rows);
            }
            'R' => {
                c = migrate(r, c, l, W, &rows);
            }
            'U' => {
                r = migrate(c, r, -l, H, &cols);
            }
            'D' => {
                r = migrate(c, r, l, H, &cols);
            }
            _ => (),
        }
        println!("{} {}", r, c);
    }
}

fn migrate(
    idx: isize,
    cur: isize,
    diff: isize,
    max: isize,
    map: &BTreeMap<isize, BTreeSet<isize>>,
) -> isize {
    let inf = *map
        .get(&idx)
        .and_then(|blocks| blocks.range(..cur).next_back())
        .unwrap_or(&0);
    let sup = *map
        .get(&idx)
        .and_then(|blocks| blocks.range(cur + 1..).next())
        .unwrap_or(&(max + 1));
    debug!(idx, cur, diff, inf, sup);
    (diff + cur).max(inf + 1).min(sup - 1)
}

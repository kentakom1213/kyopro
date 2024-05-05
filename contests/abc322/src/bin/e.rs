// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn add_vec(a: &[usize], b: &[usize], P: usize) -> Vec<usize> {
    a.iter().zip(b.iter()).map(|(&x, &y)| P.min(x + y)).collect()
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

fn main() {
    input! {
        N: usize,
        K: usize,
        P: usize,
    }

    let mut costs = vec![];
    let mut edges = vec![];

    for _ in 0..N {
        input! {
            c: usize,
            edge: [usize; K],
        }
        costs.push(c);
        edges.push(edge);
    }

    // 最短経路探索
    let mut dist = HashMap::new();
    let start = vec![0; K];
    dist.insert(start.clone(), 0);

    for i in 0..N {
        let edge = &edges[i];
        let cost = costs[i];
        for (u, d) in dist.clone() {
            let v = add_vec(&u, &edge, P);
            chmin! {
                *dist.entry(v.clone()).or_insert(INF),
                d + cost
            };
        }
    }

    if let Some(x) = dist.get(&vec![P; K]) {
        println!("{}", x);
    } else {
        println!("-1");
    }
}

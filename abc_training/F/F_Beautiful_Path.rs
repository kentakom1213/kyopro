//            F - Beautiful Path           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc324/tasks/abc324_f
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![warn(clippy::default_numeric_fallback)]

use std::collections::VecDeque;

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Usize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

/// `a < b` のとき、`a`を`b`に置き換え、trueを返す
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

type Graph = Vec<Vec<(usize, usize, usize)>>;
const EPS: f64 = 1e-9;

fn main() {
    input! {
        N: usize,
        M: usize,
    }

    // 逆向きに辺を張る
    let G = {
        let mut G = vec![vec![]; N];
        for _ in 0..M {
            input! {
                u: Usize1,
                v: Usize1,
                b: usize,
                c: usize
            }
            G[v].push((u, b, c));
        }
        G
    };

    // 2分探索
    let mut ok = 0.0;
    let mut ng = 1e9;
    while (ng - ok) > EPS {
        let mid = (ok + ng) / 2.0;
        if is_ok(mid, &G) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{:.20}", ok);
}

/// 値yを超えるようなパスが存在するか
fn is_ok(y: f64, G: &Graph) -> bool {
    let N = G.len();
    let mut W = vec![-1e20; N];
    W[0] = 0.0;

    // 重み 
    // dpで重みの和の最大値を求める
    for v in 1..N {
        for &(u, b, c) in &G[v] {
            let w = (b as f64) - y * (c as f64);
            // 重みを更新
            chmax! {
                W[v],
                W[u] + w
            };
        }
    }

    W[N - 1] >= 0.0
}

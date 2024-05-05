// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::VecDeque;

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
const MINF: isize = -(INF as isize);

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
        M: usize,
        A: [isize; N],
        edges: [(Usize1, Usize1); M]
    }

    // グラフを構築
    let mut G = vec![vec![]; N];
    for &(u, v) in &edges {
        G[u].push(v);
    }

    // 取引価格で頂点をソート
    let sorted_idx = (0..N).sorted_by_key(|&i| A[i]).collect_vec();

    let mut ans = MINF;

    let mut visited = vec![false; N];

    for &i in &sorted_idx {
        // BFS
        let mut q = VecDeque::new();
        q.push_back(i);

        while let Some(u) = q.pop_front() {
            if u != i {
                chmax! {
                    ans,
                    A[u] - A[i]
                };
            }
            if visited[u] {
                continue;
            }
            visited[u] = true;
            for &v in &G[u] {
                q.push_back(v);
            }
        }
    }

    println!("{}", ans);
}

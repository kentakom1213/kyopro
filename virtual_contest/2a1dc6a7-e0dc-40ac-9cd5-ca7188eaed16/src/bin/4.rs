// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}};

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
        edges: [(Usize1, Usize1, usize); M],
    }

    let mut edge = vec![vec![INF; N]; N];
    let mut dist = vec![vec![INF; N]; N];
    
    for &(u, v, w) in &edges {
        if u == 0 || v == 0 {
            edge[u][v] = w;
            edge[v][u] = w;
        } else {
            dist[u][v] = w;
            dist[v][u] = w;
        }
    }
    
    for k in 0..N {
        for i in 0..N {
            for j in 0..N {
                chmin! {
                    dist[i][j],
                    dist[i][k] + dist[k][j],
                };
            }
        }
    }

    // 経由地点を全探索
    let mut ans = INF;
    for i in 1..N {
        for j in i + 1..N {
            chmin! {
                ans,
                edge[0][i] + dist[i][j] + edge[j][0]
            };
            debug!(i+1, j+1, edge[0][i], dist[i][j], edge[j][0])
        }
    }

    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

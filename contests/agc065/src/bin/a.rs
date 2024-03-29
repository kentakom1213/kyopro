// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::{
    collections::{BTreeSet, BinaryHeap},
    iter::FromIterator,
};

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use rustc_hash::FxHashSet;

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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N]
    }

    let mut V = A
        .iter()
        .enumerate()
        .fold(BTreeSet::new(), |mut set, (i, &x)| {
            set.insert((x, i));
            set
        });

    // プリム法の開始頂点
    let start = 0;

    // 頂点0から出ている辺をすべて追加
    let mut edges = BinaryHeap::new();
    for i in 0..N {
        if i == start {
            continue;
        }
        edges.push(((K + A[i] - A[start]) % K, start, i));
    }

    let mut indeg = vec![false; N];
    let mut outdeg = vec![false; N];
    let mut used_edges = vec![];
    let mut tmp = 0;

    // プリム法
    while let Some((w, u, v)) = edges.pop() {
        if used_edges.len() == N {
            break;
        }
        if !outdeg[u] && !indeg[v] {
            tmp += w;
            outdeg[u] = true;
            indeg[v] = true;
            used_edges.push((u, v));
            V.remove(&(w, u));
            // vから出る辺を追加
            if let Some(&(_, i)) = V.range(((K + A[v] - 1) % K, 0)..).next() {
                edges.push(((K + A[i] - A[v]) % K, v, i));
            }
            if let Some(&(_, i)) = V.range(..(A[v], 0)).next_back() {
                edges.push(((K + A[i] - A[v]) % K, v, i));
            }
            debug!(edges);
        }
    }

    debug!(indeg);
    debug!(outdeg);

    let mut ans = 0;

    // 辺を消していく
    for &(u, v) in &used_edges {
        chmax! {
            ans,
            tmp - (K + A[v] - A[u]) % K
        };
    }

    println!("{}", ans);
}

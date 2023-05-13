//            E - Ranges on Tree           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc240/tasks/abc240_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

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

// main
fn main() {
    input! {
        N: usize,
        edges: [(Usize1, Usize1); N - 1],
    }

    let mut G = vec![vec![]; N];
    for &(u, v) in &edges {
        G[u].push(v);
        G[v].push(u);
    }

    // 区間の列
    let mut ranges = vec![(INF, INF); N];
    ranges[0].0 = 1;

    // 木DP
    rec(INF, 0, 1, &G, &mut ranges);

    // 表示
    for &(l, r) in &ranges {
        println!("{} {}", l, r);
    }
}

fn rec(p: usize, u: usize, l: usize, G: &Vec<Vec<usize>>, ranges: &mut Vec<(usize, usize)>) {
    debug!(u, &ranges);
    // 葉である場合
    if G[u].len() == 1 && G[u][0] == p {
        ranges[u].0 = l;
        ranges[u].1 = l;
    }

    let mut i = 0;
    for &v in &G[u] {
        if v != p {
            ranges[v].0 = l + i;
            rec(u, v, l + i, G, ranges);
            ranges[u].1 = ranges[v].1;
            i += 1;
        }
    }
}

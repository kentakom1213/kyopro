// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use superslice::Ext;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}
macro_rules! debug2D {
    ( $array:expr ) => {{
        #![cfg(debug_assertions)]
        eprintln!("{}: ", stringify!($array));
        for row in &$array {
            eprintln!("{:?}", row);
        }
    }};
}

#[fastout]
fn main() {
    input! {
        N: usize,
        P: [Usize1; N - 1],
        Q: usize,
        UD: [(Usize1, usize); Q],
    }

    // グラフを構築
    let G = (1..).zip(&P).fold(vec![vec![]; N], |mut g, (i, &p)| {
        g[p].push(i);
        g[i].push(p);
        g
    });

    debug2D!(G);

    // オイラーツアー
    let mut in_array = vec![INF; N];
    let mut out_array = vec![INF; N];
    in_array[0] = 0;
    let mut depth = vec![vec![]; N];
    euler_tour(
        INF,
        0,
        &mut 0,
        0,
        &G,
        &mut in_array,
        &mut out_array,
        &mut depth,
    );

    debug!(in_array);
    debug!(out_array);
    debug2D!(depth);

    // クエリの処理
    for &(u, d) in &UD {
        let (in_id, out_id) = (in_array[u], out_array[u]);
        let start = depth[d].lower_bound(&in_id);
        let end = depth[d].lower_bound(&out_id);
        println!("{}", end - start);
    }
}

fn euler_tour(
    prv: usize,
    cur: usize,
    id: &mut usize,
    d: usize,
    G: &Vec<Vec<usize>>,
    in_array: &mut [usize],
    out_array: &mut [usize],
    depth: &mut Vec<Vec<usize>>,
) {
    in_array[cur] = *id;
    depth[d].push(*id);
    for &nxt in &G[cur] {
        if nxt == prv {
            continue;
        }
        *id += 1;
        euler_tour(cur, nxt, id, d + 1, G, in_array, out_array, depth);
    }
    *id += 1;
    out_array[cur] = *id;
}

const INF: usize = 1001001001001001001;

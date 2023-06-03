// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::{VecDeque, BTreeSet};

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
type Graph = Vec<Vec<usize>>;

// main
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        edges: [(Usize1, Usize1); M],
        K: usize,
        xy: [(Usize1, Usize1); K],
        Q: usize,
        pq: [(Usize1, Usize1); Q],
    }

    // グラフの構築
    let mut G = vec![vec![]; N];
    for &(u, v) in &edges {
        G[u].push(v);
        G[v].push(u);
    }

    // グラフを連結成分ごとに色分け
    let comp = {
        let mut comp = vec![INF; N];
        let mut c = 1_usize.wrapping_neg();
        for i in 0..N {
            let mut q = VecDeque::new();
            q.push_back(i);
            if comp[i] == INF {
                c = c.wrapping_add(1);
                comp[i] = c;
            }
            while let Some(u) = q.pop_front() {
                for &v in &G[u] {
                    if comp[v] == INF {
                        comp[v] = c;
                        q.push_back(v);
                    }
                }
            }
        }
        
        comp
    };

    debug!(&comp);

    // いいグラフの条件を更新
    let mut bad = BTreeSet::new();
    for &(u, v) in &xy {
        let mut new_u = comp[u];
        let mut new_v = comp[v];
        if new_u > new_v {
            std::mem::swap(&mut new_u, &mut new_v);
        }
        bad.insert((new_u, new_v));
    }

    debug!(&bad);

    // クエリを圧縮して検索
    for &(u, v) in &pq {
        let mut new_u = comp[u];
        let mut new_v = comp[v];
        if new_u > new_v {
            std::mem::swap(&mut new_u, &mut new_v);
        }
        debug!(new_u, new_v);
        if bad.contains(&(new_u, new_v)) {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};
use std::collections::BTreeSet;

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
#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
    }
    let queries = {
        let mut qs = vec![];
        for _ in 0..Q {
            input!{q: usize}
            if q == 1 {
                input! {
                    u: Usize1,
                    v: Usize1
                }
                qs.push((1, u, v));
            }
            else {
                input! {
                    v: Usize1,
                }
                qs.push((2, v, INF));
            }
        }
        qs
    };

    debug!(&queries);

    // つける，切るが償却O(N + M)でいける気がする
    let mut G = vec![BTreeSet::<usize>::new(); N];
    let mut count = N;

    for &(q, u, v) in &queries {
        if q == 1 {
            if G[u].is_empty() {
                count -= 1;
            }
            G[u].insert(v);
            if G[v].is_empty() {
                count -= 1;
            }
            G[v].insert(u);
        }
        else {
            let vs = G[u].iter().cloned().collect_vec();
            for v in vs {
                if G[v].len() == 1 {
                    count += 1;
                }
                G[v].remove(&u);
            }
            if !G[u].is_empty() {
                count += 1;
            }
            G[u].clear();
        }
        debug!(&G);
        println!("{}", count);
    }
}

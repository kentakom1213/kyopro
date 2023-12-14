// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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

fn main() {
    input! {
        N: usize,
        A: [Usize1; N]
    }

    let mut indeg = vec![0_usize; N];
    let mut G = vec![vec![]; N];

    for (i, &a) in A.iter().enumerate() {
        indeg[a] += 1;
        G[i].push(a);
    }

    // ループに含まれている数字は取ることができる
    let mut leaf = (0..N).filter(|&i| indeg[i] == 0).collect_vec();

    let mut no = 0;

    while let Some(u) = leaf.pop() {
        no += 1;
        for &v in &G[u] {
            indeg[v] -= 1;
            if indeg[v] == 0 {
                leaf.push(v);
            }
        }
    }

    let ans = N - no;

    println!("{ans}");
}

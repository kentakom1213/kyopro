//             F - Endless Walk            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc245/tasks/abc245_f
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
type Graph = Vec<Vec<usize>>;

// main
fn main() {
    input! {
        N: usize,
        M: usize,
        edges: [(Usize1, Usize1); M],
    }

    let mut G: Graph = vec![vec![]; N];
    let mut revG: Graph = vec![vec![]; N]; // 逆向きのグラフ

    for &(u, v) in &edges {
        G[u].push(v);
        revG[v].push(u);
    }

    // 出次数が0のものを格納
    let mut out_deg = vec![0; N];
    let mut que = vec![];
    for i in 0..N {
        out_deg[i] = G[i].len();
        if out_deg[i] == 0 {
            que.push(i);
        }
    }

    debug!(&que);

    // 条件を満たさない頂点
    let mut not_ok = 0;

    // 出次数0の頂点から順に消していく
    while let Some(u) = que.pop() {
        not_ok += 1;
        // uに入ってきている点を調べる
        for &v in &revG[u] {
            out_deg[v] -= 1;
            if out_deg[v] == 0 {
                que.push(v);
            }
        }
    }

    println!("{}", N - not_ok);
}

#![allow(non_snake_case)]

use std::usize;

use cp_library_rs::{chmax, chmin, debug};
use proconio::{input, marker::Usize1};

type Graph = Vec<Vec<(usize, usize)>>;

fn main() {
    input! {
        N: usize,
        M: usize,
        L: usize,
        S: usize,
        T: usize,
        UVC: [(Usize1, Usize1, usize); M]
    }

    let G = UVC.iter().fold(vec![vec![]; N], |mut g, &(u, v, c)| {
        g[u].push((v, c));
        g
    });

    let mut isok = vec![false; N];

    dfs(&G, &mut isok, (S, T), 0, L, 0);

    debug!(isok,);

    for v in 0..N {
        if isok[v] {
            print!("{} ", v + 1);
        }
    }
    println!();
}

fn dfs(G: &Graph, isok: &mut [bool], (S, T): (usize, usize), u: usize, cnt: usize, weight: usize) {
    if cnt == 0 {
        isok[u] |= S <= weight && weight <= T;
        return;
    }
    for &(v, c) in &G[u] {
        dfs(G, isok, (S, T), v, cnt - 1, weight + c);
    }
}

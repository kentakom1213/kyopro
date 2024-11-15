#![allow(non_snake_case)]

use cp_library_rs::{chmin, debug, debug2D, utils::consts::INF};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        Mg: usize,
        UV: [(Usize1, Usize1); Mg],
        Mh: usize,
        AB: [(Usize1, Usize1); Mh],
    }

    let mut A = vec![vec![INF; N]; N];
    for i in 0..N {
        input! {
            row: [usize; N - i - 1]
        }
        for (j, &a) in row.iter().enumerate() {
            A[i][i + j + 1] = a;
            A[i + j + 1][i] = a;
        }
    }
    debug2D!(A);

    let G = UV.iter().fold(vec![vec![false; N]; N], |mut g, &(u, v)| {
        g[u][v] = true;
        g[v][u] = true;
        g
    });

    let H = AB.iter().fold(vec![vec![false; N]; N], |mut h, &(a, b)| {
        h[a][b] = true;
        h[b][a] = true;
        h
    });

    debug2D!(G);
    debug2D!(H);

    // 順列全探索
    let mut ans = INF;

    
}

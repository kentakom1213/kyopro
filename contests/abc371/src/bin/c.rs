#![allow(non_snake_case)]

use cp_library_rs::{chmin, debug, debug2D, utils::consts::INF};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        MG: usize,
        UV: [(Usize1, Usize1); MG],
        MH: usize,
        AB: [(Usize1, Usize1); MH],
    }

    let A = (1..N)
        .rev()
        .map(|i| {
            input! {a: [usize; i]};
            a
        })
        .collect_vec();

    debug2D!(A);

    // 隣接リストを構築
    let G = {
        let mut g = vec![vec![false; N]; N];
        for &(u, v) in &UV {
            g[u][v] = true;
            g[v][u] = true;
        }
        g
    };
    let H = {
        let mut h = vec![vec![false; N]; N];
        for &(a, b) in &AB {
            h[a][b] = true;
            h[b][a] = true;
        }
        h
    };

    let cmb = {
        let mut cmb = vec![];
        for i in 0..N {
            for j in i + 1..N {
                cmb.push((i, j));
            }
        }
        cmb
    };

    let cost = {
        let mut c = vec![vec![INF; N]; N];
        for i in 0..N - 1 {
            for j in i + 1..N {
                c[i][j] = A[i][j - i - 1];
                c[j][i] = A[i][j - i - 1];
            }
        }
        c
    };

    debug2D!(cost);
    debug!(cmb);

    // Hを全探索
    let mut ans = INF;

    for perm in (0..N).permutations(N) {
        let mut tmp = 0;

        for a in 0..N {
            for b in a + 1..N {
                let u = perm[a];
                let v = perm[b];

                if H[u][v] != G[a][b] {
                    tmp += cost[u][v];
                }
            }
        }

        chmin!(ans, tmp);
    }

    println!("{ans}");
}

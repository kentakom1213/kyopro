#![allow(non_snake_case)]

use cp_library_rs::{chmin, debug, debug2D, utils::consts::INF};
use itertools::*;
use proconio::{fastout, input, marker::Usize1};
use rustc_hash::FxHashSet;

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        UVT: [(Usize1, Usize1, usize); M],
    }

    // 最短路を構成
    let mut dist = vec![vec![INF; N]; N];

    for i in 0..N {
        dist[i][i] = 0;
    }

    for &(u, v, t) in &UVT {
        chmin! {
            dist[u][v],
            t
        };
        chmin! {
            dist[v][u],
            t
        };
    }

    for k in 0..N {
        for i in 0..N {
            for j in 0..N {
                chmin! {
                    dist[i][j],
                    dist[i][k] + dist[k][j],
                };
            }
        }
    }

    debug2D!(dist);

    // クエリ処理
    input! {
        Q: usize,
    }

    for _ in 0..Q {
        input! {
            K: usize,
            B: [Usize1; K],
        }

        // 橋の重みの総和
        let W = B.iter().map(|&e| UVT[e].2).sum::<usize>();
        let mut ans = INF;

        // 端点の組合せを全探索
        for P in (0..K).permutations(K) {
            for S in 0..1 << K {
                // 頂点列を構成
                let mut vs = vec![0];
                for i in 0..K {
                    let (mut u, mut v, _) = UVT[B[P[i]]];
                    if S >> i & 1 == 1 {
                        (u, v) = (v, u);
                    }
                    vs.push(u);
                    vs.push(v);
                }
                vs.push(N - 1);

                debug!(vs);

                // 距離
                let mut tmp = W;

                for i in 0..K + 1 {
                    let u = vs[2 * i];
                    let v = vs[2 * i + 1];
                    debug!(u, v);
                    tmp += dist[u][v];
                }

                chmin! {
                    ans,
                    tmp,
                };

                debug!(tmp);
            }
        }

        println!("{ans}");
    }
}

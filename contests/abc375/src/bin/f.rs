#![allow(non_snake_case)]

use cp_library_rs::{chmin, utils::consts::INF};
use itertools::Itertools;
use proconio::{input, marker::Usize1};
use rustc_hash::FxHashSet;

fn main() {
    input! {
        N: usize,
        M: usize,
        Q: usize,
        ABC: [(Usize1, Usize1, usize); M],
    }

    let mut stopped = FxHashSet::default();

    // クエリ先読み
    let queries = (0..Q)
        .map(|_| {
            input!(t: usize);
            if t == 1 {
                input!(x: Usize1);
                stopped.insert(x);
                Query::Stop(x)
            } else {
                input!(x: Usize1, y: Usize1);
                Query::Dist(x, y)
            }
        })
        .collect_vec();

    // 通行止めになっていない辺のみを使ってFloyd-Warshall
    let mut dist = vec![vec![INF; N]; N];

    for i in 0..N {
        dist[i][i] = 0;
    }

    for (i, &(a, b, c)) in ABC.iter().enumerate() {
        if !stopped.contains(&i) {
            dist[a][b] = c;
            dist[b][a] = c;
        }
    }

    for k in 0..N {
        for i in 0..N {
            for j in 0..N {
                chmin!(dist[i][j], dist[i][k] + dist[k][j]);
            }
        }
    }

    // クエリを逆順に処理
    let mut ans = vec![];
    for q in queries.into_iter().rev() {
        match q {
            Query::Stop(n) => {
                // iを解放
                let (a, b, c) = ABC[n];
                chmin!(dist[a][b], c);
                chmin!(dist[b][a], c);

                // もう一回
                for i in 0..N {
                    for j in 0..N {
                        chmin!(
                            dist[i][j],
                            dist[i][a] + dist[a][b] + dist[b][j],
                            dist[i][b] + dist[b][a] + dist[a][j]
                        );
                    }
                }
            }
            Query::Dist(a, b) => {
                ans.push(dist[a][b]);
            }
        }
    }

    ans.reverse();

    for &d in &ans {
        if d == INF {
            println!("-1");
        } else {
            println!("{d}");
        }
    }
}

enum Query {
    Stop(usize),
    Dist(usize, usize),
}

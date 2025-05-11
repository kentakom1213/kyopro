#![allow(non_snake_case)]

use cp_library_rs::{debug, graph::lca_doubling::LCA};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(Usize1, Usize1); M],
        Q: usize,
        CD: [(Usize1, Usize1); Q],
    }

    // AB から木を構築
    let ranges = AB
        .iter()
        .enumerate()
        .flat_map(|(i, &(a, b))| [(a, i + 1), (b, i + 1)])
        .chain([(0, 0), (2 * N, 0)])
        .sorted()
        .collect_vec();

    debug!(ranges);

    let mut G = vec![vec![]; N];

    // どの頂点に属するか
    let vertex = vec![0; 2 * N];
    let mut st = vec![];

    for &(_, q) in &ranges {
        if let Some(&p) = st.last() {
            if p == q {
                // backtrack
                st.pop();
            } else {
                st.push(q);
                // 辺を張る
                G[p].push(q);
                G[q].push(p);
            }
        } else {
            st.push(q);
        }
        debug!(st);
    }

    debug!(G);

    let lca = LCA::new(&G, 0);

    // クエリに答える

}

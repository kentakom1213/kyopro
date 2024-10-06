#![allow(non_snake_case)]

use ac_library::mincostflow::*;
use cp_library_rs::{debug, utils::consts::IINF};
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        N: usize,
        UVH: [(usize, usize, isize); N]
    }

    // 座圧
    let (mut U, mut V): (Vec<usize>, Vec<usize>) = UVH.iter().map(|&(u, v, _)| (u, v)).unzip();
    U.sort();
    U.dedup();
    V.sort();
    V.dedup();

    debug!(U);
    debug!(V);

    let X = U.len();
    let Y = V.len();
    let (S, T) = (X + Y, X + Y + 1);

    let mut flow = MinCostFlowGraph::new(X + Y + 2);

    for &(u, v, h) in &UVH {
        let idx_u = U.lower_bound(&u);
        let idx_v = V.lower_bound(&v) + X;

        flow.add_edge(idx_u, idx_v, 1, IINF - h);
    }

    for idx_u in 0..X {
        flow.add_edge(S, idx_u, 1, 0);
    }

    for idx_v in X..X + Y {
        flow.add_edge(idx_v, T, 1, 0);
    }

    // 流す
    flow.flow(S, T, IINF);

    // 辺を取得
    let mut res = vec![];

    for (i, &(u, v, _)) in UVH.iter().enumerate() {
        let edge = flow.get_edge(i);

        debug!(u, v, edge.from, edge.to, edge.flow);

        if edge.flow >= 1 {
            res.push((v, u));
        }
    }

    res.sort();

    println!("{}", res.len());
    for &(v, u) in &res {
        println!("{} {}", v, u);
    }
}

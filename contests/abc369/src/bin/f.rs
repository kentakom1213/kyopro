#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::monoid::examples::Max, chmax, data_structure::segment_tree::SegmentTree,
    debug, utils::consts::INF,
};
use itertools::Itertools;
use proconio::{input, marker::Usize1};
use rustc_hash::FxHashMap;

fn main() {
    input! {
        H: usize,
        W: usize,
        N: usize,
        RC: [(Usize1, Usize1); N],
    }

    // i+jでソート
    let mut rows = (0..H).map(|i| (0, (INF, INF), (i, 0))).collect::<Vec<_>>();
    let mut cols = (0..W).map(|j| (0, (INF, INF), (0, j))).collect::<Vec<_>>();

    for &(i, j) in RC.iter().sorted_by_key(|&(i, j)| i + j) {
        if rows[i] >= cols[j] {
            // 横移動
            let (pv, _, prv) = rows[i];
            rows[i] = (pv + 1, prv, (i, j));
            cols[j] = (pv + 1, prv, (i, j));
        } else {
            // 縦移動
            let (pv, _, prv) = cols[j];
            rows[i] = (pv + 1, prv, (i, j));
            cols[j] = (pv + 1, prv, (i, j));
        }
    }

    let mut ans = 0;
    let mut cur = (0, 0);

    for &(i, j) in &RC {
        if chmax!(ans, rows[i].0, cols[j].0) {
            cur = (i, j);
        }
    }

    debug!(rows);
    debug!(cols);

    println!("{ans}");
    debug!(cur);

    let mut path_rev = vec![];

    while cur != (INF, INF) {
        let (ci, cj) = cur;
        let (pi, pj) = rows[cur.0].1;

        if pi == ci {
            path_rev.push("D".repeat(cj - pj));
        } else {
            path_rev.push("R".repeat(ci - pi));
        }

        cur = rows[pi].2;
    }

    println!("{}", path_rev.iter().rev().join(""));
}

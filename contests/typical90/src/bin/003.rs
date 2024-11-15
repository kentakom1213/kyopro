#![allow(non_snake_case)]

use cp_library_rs::{debug, graph::dynamic_rerooting::Rerooting};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        AB: [(Usize1, Usize1); N - 1]
    }

    let mut dp: Rerooting<usize, _, _, _> =
        Rerooting::new(N, 0, |a, b| *a.max(b), |x, _| x + 1, |x, _| *x);

    for &(u, v) in &AB {
        dp.add_edge2(u, v);
    }

    // 集約
    dp.build();

    debug!(dp.ans);

    let ans = *dp.ans.iter().max().unwrap() + 1;

    println!("{ans}");
}

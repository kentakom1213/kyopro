#![allow(non_snake_case)]

use cp_library_rs::{debug, graph::dynamic_rerooting::Rerooting};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        M: usize,
        XY: [(Usize1, Usize1); N - 1]
    }

    let mut tree = Rerooting::new(
        N,
        1, // [白, 黒]
        |&x, &y| x * y % M,
        |&val, _| val + 1,
        |&val, _| val,
    );

    for &(x, y) in &XY {
        tree.add_edge2(x, y);
    }

    tree.build();

    debug!(tree.dp);
    debug!(tree.ans);

    println!("{}", tree.ans.iter().join("\n"));
}

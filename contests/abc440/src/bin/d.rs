#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::operation::Add, data_structure::segment_tree::SegmentTree, debug,
    utils::consts::INF,
};
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        Q: usize,
        mut A: [usize; N],
        XY: [(usize, usize); Q]
    }
    let A = {
        A.sort();
        A
    };

    let S = {
        let mut s = vec![(0, 0)];
        for &a in &A {
            let p = *s.last().unwrap();
            if p.1 + 1 == a {
                s.last_mut().unwrap().1 = a;
            } else {
                s.push((a, a));
            }
        }
        s.push((INF, INF));
        s
    };
    debug!(S);

    // 逆
    let X: Vec<_> = S
        .iter()
        .tuple_windows()
        .map(|(&(_, p), &(n, _))| (p + 1, n - 1))
        .collect();
    debug!(X);

    let seg: SegmentTree<Add<usize>> = X.iter().map(|&(l, r)| r + 1 - l).collect();
    debug!(seg);

    for &(x, y) in &XY {
        let lidx = X.partition_point(|v| v.1 < x);
        debug!(x, lidx, &X[..lidx], &X[lidx..]);

        let offset = x.saturating_sub(X[lidx].0);
        debug!(offset);

        let (sum, ridx) = seg.max_right(lidx, |s| s < y + offset);
        debug!(ridx);

        let tidx = y + offset - sum;
        let ans = X[ridx].0 + tidx - 1;

        println!("{ans}");
    }
}

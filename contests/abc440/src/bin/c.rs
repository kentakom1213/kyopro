#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::operation::Add, chmin, data_structure::segment_tree::SegmentTree, debug,
    utils::consts::INF,
};
use proconio::input;

fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        solve()
    }
}

fn solve() {
    input! {
        N: usize,
        W: usize,
        C: [usize; N]
    }

    let mut seg = SegmentTree::<Add<usize>>::new(4 * W);

    for i in 0..N {
        *seg.get_mut(i % (2 * W)).unwrap() += C[i];
        *seg.get_mut(i % (2 * W) + 2 * W).unwrap() += C[i];
    }

    debug!(seg);

    let mut ans = INF;

    // 半分足す
    for l in 0..2 * W {
        let tmp = seg.get_range(l..l + W);

        chmin!(ans, tmp);
    }

    println!("{ans}");
}

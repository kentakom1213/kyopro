#![allow(non_snake_case)]

use std::cmp::Reverse;

use cp_library_rs::{
    algebraic_structure::operation::Add, data_structure::segment_tree::SegmentTree, debug,
};
use proconio::input;

fn main() {
    input! {
        N: usize,
        K: usize,
        X: usize,
        mut A: [usize; N]
    }
    A.sort_by_key(|a| Reverse(*a));

    let seg = SegmentTree::<Add<usize>>::from_vec(A);

    let (s, i) = seg.max_right(N - K, |v| v < X);

    debug!(N - K, X, s, i);

    if i < N && seg.get_range(N - K..=i) >= X {
        println!("{}", i + 1);
    } else {
        println!("-1");
    }
}

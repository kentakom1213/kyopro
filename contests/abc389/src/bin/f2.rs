#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::actedmonoid::examples::AddMax,
    data_structure::lazy_segment_tree::LazySegmentTree, utils::show_binary_tree::ShowBinaryTree,
};
use proconio::{fastout, input};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        N: usize,
        LR: [(usize, usize); N],
        Q: usize,
        X: [usize; Q]
    }

    let X_sorted = {
        let mut x = X.clone();
        x.sort_unstable();
        x
    };

    let mut seg = LazySegmentTree::<AddMax<usize>>::from_vec(X_sorted.clone());

    seg.print_as_binary_tree();

    for &(l, r) in &LR {
        let (_, lb) = seg.max_right(0, |v| v < l);
        let (_, ub) = seg.max_right(0, |v| v <= r);

        seg.apply(lb..ub, 1);

        seg.print_as_binary_tree();
    }

    for &x in &X {
        let idx = X_sorted.lower_bound(&x);
        let res = seg.get(idx..=idx);

        println!("{res}");
    }
}

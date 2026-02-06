#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::{operation::Add, to_acted::ToActed},
    data_structure::dynamic_segment_tree::DynamicSegmentTree,
};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        X: [i64; N],
        P: [usize; N],
        Q: usize,
        LR: [(i64, i64); Q]
    }

    let mut seg = DynamicSegmentTree::<i64, ToActed<Add<usize>>>::new(-1001001001, 1001001001);

    for (&x, &p) in X.iter().zip(&P) {
        seg.update(x, p);
    }

    for &(l, r) in &LR {
        let ans = seg.get_range(l..=r);
        println!("{ans}");
    }
}

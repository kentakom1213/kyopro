#![allow(non_snake_case)]

use std::cmp::Reverse;

use cp_library_rs::{
    algebraic_structure::operation::Max, data_structure::dynamic_segment_tree::DynamicSegmentTree,
};
use proconio::input;

fn main() {
    input! {
        N: usize,
        mut AB: [(usize, usize); N]
    }

    AB.sort_by_key(|&(a, b)| (a, Reverse(b)));

    // B の LIS を求める
    let mut seg = DynamicSegmentTree::<usize, Max<usize>>::default();

    for &(_, b) in &AB {
        let max = seg.get_range(..b);
        *seg.get_mut(b) = max + 1;
    }

    let ans = seg.get_range(..);

    println!("{ans}");
}

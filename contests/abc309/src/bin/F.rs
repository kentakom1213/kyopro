#![allow(non_snake_case)]

use std::cmp::Reverse;

use cp_library_rs::{
    algebraic_structure::{operation::Max, to_acted::ToActed},
    chmax,
    data_structure::dynamic_segment_tree::DynamicSegmentTree,
    debug,
};
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        HWD: [(usize, usize, usize); N]
    }

    let HWD = HWD
        .into_iter()
        // h <= w <= d
        .map(|(h, w, d)| {
            let mut arr = [h, w, d];
            arr.sort();
            arr
        })
        // d の降順にソート
        .sorted_by_key(|&[h, w, d]| (Reverse(d), w, h))
        .collect_vec();

    let mut seg = DynamicSegmentTree::<usize, ToActed<Max<usize>>>::new(0, 1001001001);

    for &[h, w, _d] in &HWD {
        let max_h = seg.get_range(w + 1..);

        debug!(h, w, _d, max_h);

        if max_h > h {
            println!("Yes");
            return;
        }

        // 追加
        chmax!(*seg.get_mut(w).unwrap(), h);
    }

    println!("No");
}

#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::operation::Max, data_structure::dynamic_segment_tree::DynamicSegmentTree,
};
use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    // 最長非増加部分列を求める
    let mut seg = DynamicSegmentTree::<usize, Max<usize>>::default();

    for &a in &A {
        let max = seg.get_range(a..);
        *seg.get_mut(a) = max + 1;

        seg.print_as_binary_tree();
    }

    let ans = seg.get_range(..);

    println!("{ans}");
}

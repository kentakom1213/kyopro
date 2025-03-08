#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::operation::Xor, data_structure::segment_tree::SegmentTree,
    utils::show_binary_tree::ShowBinaryTree,
};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [usize; N],
    }

    let mut seg = SegmentTree::<Xor>::from(A);

    seg.print_as_binary_tree();

    for _ in 0..Q {
        input! {
            t: usize,
            x: Usize1,
            y: usize,
        }

        if t == 1 {
            *seg.get_mut(x).unwrap() ^= y;

            seg.print_as_binary_tree();
        } else {
            let res = seg.get_range(x..y);

            println!("{res}");
        }
    }
}

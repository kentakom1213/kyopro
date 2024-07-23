#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::monoid::examples::Xor, data_structure::segment_tree::SegmentTree,
};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [usize; N],
    }

    let mut seg = SegmentTree::<Xor>::from(&A);

    for _ in 0..Q {
        input! {
            t: usize,
            x: Usize1,
            y: usize,
        }

        if t == 1 {
            *seg.get_mut(x).unwrap() ^= y;
        } else {
            let res = seg.get_range(x..y);

            println!("{res}");
        }
    }
}

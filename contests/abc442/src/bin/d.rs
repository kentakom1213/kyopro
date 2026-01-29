#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::operation::Add, data_structure::segment_tree::SegmentTree,
};
use proconio::input;

fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [usize; N]
    }

    let mut seg = SegmentTree::<Add<usize>>::from_vec(A);

    for _ in 0..Q {
        input! {
            t: usize,
        }

        if t == 1 {
            input! {
                x: usize,
            }
            let a = seg[x - 1];
            let b = seg[x];

            *seg.get_mut(x - 1).unwrap() = b;
            *seg.get_mut(x).unwrap() = a;
        } else {
            input! {
                l: usize,
                r: usize
            }

            let sum = seg.get_range(l - 1..r);

            println!("{sum}");
        }
    }
}

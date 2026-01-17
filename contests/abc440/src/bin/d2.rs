#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::actedmonoid::examples::AddSum,
    data_structure::dynamic_segment_tree::DynamicSegmentTree,
};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [usize; N],
    }

    let mut seg = DynamicSegmentTree::<usize, AddSum<i32>>::new(0, 2001001001);

    seg.apply(.., 1);

    for &a in &A {
        seg.apply(a..=a, -1);
    }

    for _ in 0..Q {
        input! {
            x: usize,
            y: i32,
        }
        let ans = seg.max_right(x, |sum| sum.0 < y);

        println!("{}", ans.1);
    }
}

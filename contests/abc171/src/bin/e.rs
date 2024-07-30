#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::monoid::examples::Xor, data_structure::segment_tree::SegmentTree,
    utils::iterutil::IterUtil,
};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    let seg = SegmentTree::<Xor>::from(&A);

    println!(
        "{}",
        (0..N)
            .map(|i| seg.get_range(..i) ^ seg.get_range(i + 1..))
            .join(" ")
    );
}

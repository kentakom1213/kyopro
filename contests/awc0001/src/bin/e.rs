#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::operation::MinMax, chmax, data_structure::segment_tree::SegmentTree,
};
use proconio::input;

fn main() {
    input! {
        N: usize,
        K: usize,
        H: [i64; N]
    }

    let seg = SegmentTree::<MinMax<i64>>::from_vec(H.iter().map(|&x| (x, x)).collect());

    let mut ans = 0;

    for i in 0..N - K + 1 {
        let (min, max) = seg.get_range(i..i + K);

        chmax!(ans, max - min);
    }

    println!("{ans}");
}

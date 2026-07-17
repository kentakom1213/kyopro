#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::operation::Max, data_structure::segment_tree::SegmentTree, debug,
};
use proconio::input;

fn main() {
    input! {
        N: usize,
        H: [usize; N],
        A: [usize; N],
    }

    // dp[h] := 末尾に高さ h の花があるときの美しさの総和の最大値
    let mut dp = SegmentTree::<Max<usize>>::new(N + 1);

    for i in 0..N {
        let max = dp.get_range(..H[i]);
        *dp.get_mut(H[i]).unwrap() = max + A[i];

        debug!(dp);
    }

    let ans = dp.get_range(..);

    println!("{ans}");
}

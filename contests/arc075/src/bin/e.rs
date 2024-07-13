#![allow(non_snake_case)]

use cp_library_rs::{debug, dynamic_segment_tree::DynamicSegmentTree, monoid_examples::Add};
use proconio::input;

fn main() {
    input! {
        N: usize,
        K: isize,
        A: [isize; N]
    }

    // 累積和
    let mut S = vec![0; N + 1];

    for i in 0..N {
        S[i + 1] = S[i] + A[i];
    }

    // S[j] - K*j >= S[i] - K*i
    // を満たすような (i,j) の数え上げ
    let mut seg = DynamicSegmentTree::<isize, Add>::new();

    *seg.get_mut(0) = 1;

    let mut ans = 0;

    // 平面走査
    for i in 1..=N {
        let x = S[i] - K * i as isize;

        ans += seg.get_range(..=x);

        *seg.get_mut(x) += 1;

        seg.print_as_binary_tree();
    }

    debug!(ans);

    println!("{ans}");
}

#![allow(non_snake_case)]

use std::ops::Bound;

use cp_library_rs::{
    algebraic_structure::operation::Add, data_structure::dynamic_segment_tree::DynamicSegmentTree,
    debug, number_theory::modint::M998,
};
use proconio::input;

fn main() {
    input! {
        N: usize,
        K: isize,
        A: [isize; N]
    }

    // 累積和
    let S = A.iter().fold(vec![0], |mut s, &a| {
        let sum = s.last().unwrap();
        s.push(sum + a);
        s
    });

    debug!(S);

    let mut seg = DynamicSegmentTree::<isize, Add<M998>>::default();

    *seg.get_mut(0) += 1;

    for i in 1..=N {
        let ng = S[i] - K;
        let tmp = seg.get_range(..ng) + seg.get_range((Bound::Excluded(ng), Bound::Unbounded));
        *seg.get_mut(S[i]) += tmp;

        if i == N {
            println!("{tmp}");
        }

        seg.print_as_binary_tree();
    }
}

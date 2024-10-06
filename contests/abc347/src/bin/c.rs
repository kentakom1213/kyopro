#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::monoid::examples::Add,
    data_structure::dynamic_segment_tree::DynamicSegmentTree,
};
use proconio::input;

fn main() {
    input! {
        N: usize,
        A: isize,
        B: isize,
        mut D: [isize; N]
    }

    let AB = A + B;

    // modを数える
    let mut seg = DynamicSegmentTree::<isize, Add<usize>>::default();

    for &d in &D {
        *seg.get_mut(d % AB) += 1;
        *seg.get_mut(d % AB + AB) += 1;
    }

    seg.print_as_binary_tree();

    // 各日付をスタートにする
    for &d in &D {
        let start = d % AB;
        let end = start + A;
        let cnt = seg.get_range(start..end);

        if cnt >= N {
            println!("Yes");
            return;
        }
    }

    println!("No");
}

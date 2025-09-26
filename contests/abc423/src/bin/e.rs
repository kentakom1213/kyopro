#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::operation::Add, data_structure::segment_tree::SegmentTree,
};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [isize; N],
        LR: [(usize, usize); Q]
    }

    let P = SegmentTree::<Add<isize>>::from_iter(A.iter().copied());
    let Q = SegmentTree::<Add<isize>>::from_iter((1..).zip(&A).map(|(i, &a)| i * a));
    let R = SegmentTree::<Add<isize>>::from_iter((1..).zip(&A).map(|(i, &a)| i * i * a));

    for &(l, r) in &LR {
        let mut ans = 0;

        ans -= R.get_range(l - 1..r);
        ans += (l + r) as isize * Q.get_range(l - 1..r);
        ans += (1 - l as isize) * (r as isize + 1) * P.get_range(l - 1..r);

        println!("{ans}");
    }
}

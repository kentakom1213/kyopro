#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::monoid::Monoid, data_structure::segment_tree::SegmentTree,
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

    let mut S = vec![0; N + 1];
    for i in 0..N {
        S[i + 1] = S[i] + A[i];
    }

    let seg = SegmentTree::<M>::from_iter(S.into_iter().map(|a| (a, 1, 0)));

    for &(l, r) in &LR {
        let ans = seg.get_range(l - 1..=r);
        println!("{}", ans.2);
    }
}

struct M;

impl Monoid for M {
    type Val = (isize, isize, isize);
    fn id() -> Self::Val {
        (0, 0, 0)
    }
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        let &(lsum, llen, lans) = left;
        let &(rsum, rlen, rans) = right;
        (
            lsum + rsum,
            llen + rlen,
            lans + rans + llen * rsum - rlen * lsum,
        )
    }
}

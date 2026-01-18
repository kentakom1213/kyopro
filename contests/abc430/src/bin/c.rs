#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::monoid::Monoid, data_structure::segment_tree::SegmentTree, debug,
};
use proconio::input;

struct Count;

impl Monoid for Count {
    type Val = (usize, usize);
    fn e() -> Self::Val {
        (0, 0)
    }
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        (left.0 + right.0, left.1 + right.1)
    }
}

fn main() {
    input! {
        N: usize,
        A: usize,
        B: usize,
        S: String
    }

    let seg = SegmentTree::<Count>::from_vec(
        S.chars()
            .map(|c| if c == 'a' { (1, 0) } else { (0, 1) })
            .collect(),
    );

    // 決め打ち
    let mut ans = 0;

    for l in 0..N {
        let min = seg.max_right(l, |v| v.0 < A);
        let max = seg.max_right(l, |v| v.1 < B);

        debug!(l, min, max);

        ans += max.1.saturating_sub(min.1);
    }

    println!("{ans}");
}

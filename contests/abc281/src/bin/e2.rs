#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::monoid::Monoid, data_structure::segment_tree::SegmentTree, debug,
    utils::show_binary_tree::ShowBinaryTree,
};
use itertools::Itertools;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
        A: [usize; N]
    }

    let comp = A.iter().copied().zip(0..).sorted().collect_vec();

    let mut seg = SegmentTree::<Cnt>::new(N);

    for (i, &a) in A[..M].iter().enumerate() {
        let idx = comp.lower_bound(&(a, i));
        *seg.get_mut(idx).unwrap() = (a, 1);
    }

    seg.print_as_binary_tree();

    for (&a, i) in A[M..].iter().zip(M..) {
        // 大きい方から K 個の和
        let res = seg.max_right(0, |(_, idx)| idx <= K);
        debug!(i, a, res);
        print!("{} ", res.0 .0);

        // 列の更新
        let pop = comp.lower_bound(&(A[i - M], i - M));
        *seg.get_mut(pop).unwrap() = (0, 0);
        let idx = comp.lower_bound(&(a, i));
        *seg.get_mut(idx).unwrap() = (a, 1);

        seg.print_as_binary_tree();
    }

    // 大きい方から K 個の和
    let res = seg.max_right(0, |(_, idx)| idx <= K);
    println!("{}", res.0 .0);
}

struct Cnt;

impl Monoid for Cnt {
    type Val = (usize, usize);
    fn e() -> Self::Val {
        (0, 0)
    }
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        (left.0 + right.0, left.1 + right.1)
    }
}

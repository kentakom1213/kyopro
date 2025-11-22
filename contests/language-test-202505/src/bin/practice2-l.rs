#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::extmonoid::ExtMonoid, data_structure::lazy_segment_tree::LazySegmentTree,
    debug, utils::show_binary_tree::ShowBinaryTree,
};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [usize; N],
        TLR: [(usize, usize, usize); Q]
    }

    let mut seg = LazySegmentTree::<XorInverse>::from(
        &A.into_iter().map(|a| (1 - a, a, 0)).collect::<Vec<_>>(),
    );

    seg.print_as_binary_tree();

    for (t, l, r) in TLR {
        if t == 1 {
            seg.apply(l - 1..r, true);
            seg.print_as_binary_tree();
        } else {
            let res = seg.get(l - 1..r);
            println!("{}", res.2);
        }
    }
}

struct XorInverse;

impl ExtMonoid for XorInverse {
    type X = (
        // 0の個数
        usize,
        // 1の個数
        usize,
        // 転倒数
        usize,
    );
    /// 反転するかどうか
    type F = bool;
    fn id_x() -> Self::X {
        (0, 0, 0)
    }
    fn id_f() -> Self::F {
        false
    }
    fn op(x: &Self::X, y: &Self::X) -> Self::X {
        let &(l0, l1, linv) = x;
        let &(r0, r1, rinv) = y;
        (l0 + r0, l1 + r1, linv + rinv + l1 * r0)
    }
    fn aggregate(x: &Self::F, _p: usize) -> Self::F {
        *x
    }
    fn composition(x: &Self::F, y: &Self::F) -> Self::F {
        x ^ y
    }
    fn mapping(x: &Self::X, y: &Self::F) -> Self::X {
        if *y {
            let &(n0, n1, inv) = x;
            return (n1, n0, n1 * n0 - inv);
        }
        *x
    }
}

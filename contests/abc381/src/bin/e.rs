#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::monoid::Monoid,
    chmax,
    data_structure::{segment_tree::SegmentTree, segment_tree_mutable::SegmentTreeMut},
    debug,
};
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        N: usize,
        Q: usize,
        S: String,
        LR: [(Usize1, usize); Q],
    }

    let seg = S
        .chars()
        .map(|c| match c {
            '1' => (1, 0, 0, 0, 0),
            '2' => (0, 1, 0, 0, 0),
            '/' => (0, 0, 1, 1, 1),
            _ => unreachable!(),
        })
        .collect::<SegmentTree<S1122>>();

    debug!(seg);
    seg.show();

    for (l, r) in LR {
        let ans = seg.get_range(l..r);
        debug!(l, r, ans);
        println!("{}", ans.4);
    }
}

struct S1122;

impl Monoid for S1122 {
    type Val = (
        usize, // 1 の個数
        usize, // 2 の個数
        usize, // 最大の '11..1/'
        usize, // 最大の '/2..22'
        usize, // 最大の '11..1/2..22'
    );
    fn id() -> Self::Val {
        (0, 0, 0, 0, 0)
    }
    fn op(lval: &Self::Val, rval: &Self::Val) -> Self::Val {
        let &(l_n1, l_n2, l_11, l_22, l_res) = lval;
        let &(r_n1, r_n2, r_11, r_22, r_res) = rval;

        let m_11 = if r_11 > 0 { l_n1 + r_11 } else { l_11 };
        let m_22 = if l_22 > 0 { l_22 + r_n2 } else { r_22 };

        let res = l_res.max(r_res).max({
            let mut ans = 0;
            if l_11 > 0 {
                chmax!(ans, (l_11 - 1).min(r_n2) * 2 + 1);
            }
            if r_22 > 0 {
                chmax!(ans, (r_22 - 1).min(r_n1) * 2 + 1);
            }
            ans
        });

        (l_n1 + r_n1, l_n2 + r_n2, m_11, m_22, res)
    }
}

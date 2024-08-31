#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::monoid::Monoid, data_structure::segment_tree::SegmentTree, debug,
    utils::iterutil::IterUtil,
};
use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        H: usize,
        W: usize,
        N: usize,
        mut RC: [(usize, usize); N]
    }

    RC.sort();
    debug!(RC);

    // cに関するLISとして考える
    let mut dp = SegmentTree::<MX>::new(W + 1);
    let mut prv = FxHashMap::default();

    for &(r, c) in &RC {
        let (maxi, pos) = dp.get_range(..=c);
        prv.insert((r, c), pos);
        *dp.get_mut(c).unwrap() = (maxi + 1, (r, c));
    }

    let (ans, (mut cr, mut cc)) = dp.get_range(..);
    let mut path_rev = vec!["D".repeat(H - cr) + &"R".repeat(W - cc)];

    while (cr, cc) != (1, 1) {
        let (pr, pc) = prv[&(cr, cc)];
        path_rev.push("D".repeat(cr - pr) + &"R".repeat(cc - pc));
        (cr, cc) = (pr, pc);
    }

    path_rev.push("D".repeat(cc - 1) + &"R".repeat(cr - 1));

    println!("{ans}");
    println!("{}", path_rev.iter().rev().join(""));
}

struct MX;
impl Monoid for MX {
    type Val = (usize, (usize, usize));
    fn id() -> Self::Val {
        (0, (1, 1))
    }
    fn op(&(lv, li): &Self::Val, &(rv, ri): &Self::Val) -> Self::Val {
        if lv >= rv {
            (lv, li)
        } else {
            (rv, ri)
        }
    }
}

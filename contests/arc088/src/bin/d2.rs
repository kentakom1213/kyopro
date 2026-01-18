#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::{commutative::CommutativeMonoid, monoid::Monoid},
    data_structure::dual_segment_tree::DualSegmentTree,
};
use proconio::input;

struct BitXor;

impl Monoid for BitXor {
    type Val = bool;
    fn e() -> Self::Val {
        false
    }
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        left ^ right
    }
}
impl CommutativeMonoid for BitXor {}

/// 観察:
/// - 単調性が成り立つ（l で OK ⇒ l-1 で OK）
/// - ⌊N/2⌋ は OK
///
/// 解の存在性で二分探索ができる？
/// → 貪欲に操作していくことで先頭 k 項の色を揃えられるか
fn main() {
    input! {
        S: String
    }

    let S: Vec<_> = S.chars().map(|c| c == '1').collect();

    let mut ok = 0;
    let mut ng = S.len() + 1;

    while ng - ok > 1 {
        let mid = (ok + ng) / 2;

        if isok(mid, &S) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{ok}");
}

fn isok(k: usize, S: &[bool]) -> bool {
    let mut seg = DualSegmentTree::<BitXor>::build(S);

    for i in 0..S.len() - k + 1 {
        if seg.get_point(i) {
            seg.apply_range(i..i + k, true);
        }
    }

    (0..k).map(|i| seg.get_point(i)).all(|f| !f)
}

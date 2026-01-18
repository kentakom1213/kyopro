#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::actedmonoid::ActedMonoid,
    data_structure::lazy_segment_tree::LazySegmentTree, debug,
    utils::show_binary_tree::ShowBinaryTree,
};
use proconio::input;

struct FlipAnd;

impl ActedMonoid for FlipAnd {
    type Val = bool;
    type Act = bool;
    fn e() -> Self::Val {
        true
    }
    fn id() -> Self::Act {
        false
    }
    fn op(x: &Self::Val, y: &Self::Val) -> Self::Val {
        x & y
    }
    fn compose(x: &Self::Act, y: &Self::Act) -> Self::Act {
        x ^ y
    }
    fn mapping(x: &Self::Val, y: &Self::Act) -> Self::Val {
        x ^ y
    }
}

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

fn isok(k: usize, S: &str) -> bool {
    debug!(k);

    let mut seg = LazySegmentTree::<FlipAnd>::from_vec(S.chars().map(|c| c == '1').collect());

    seg.print_as_binary_tree();

    for i in 0..S.len() - k + 1 {
        debug!(i, seg.get_at(i));
        if seg.get_at(i) {
            seg.apply(i..i + k, true);
        }

        seg.print_as_binary_tree();
    }

    (0..k).map(|i| seg.get_at(i)).all(|f| !f)
}

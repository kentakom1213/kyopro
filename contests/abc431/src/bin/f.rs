#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::operation::Add,
    data_structure::segment_tree::SegmentTree,
    number_theory::{comb::Comb, modint::M998},
    utils::num_traits::One,
};
use itertools::Itertools;
use proconio::input;

/// 観察:
/// - 要素 v の直前に v+D より大きい要素を置くことはできない．
fn main() {
    input! {
        N: usize,
        D: usize,
        A: [usize; N]
    }

    let cnt = {
        let mut cnt = vec![0; 1001001];
        for &a in &A {
            cnt[a] += 1;
        }
        SegmentTree::<Add<usize>>::from(cnt)
    };
    let cmb = Comb::<M998>::new(2001001);

    // 昇順に確定させていく
    let mut ans = M998::one();

    for a in A.into_iter().sorted().dedup() {
        // 箱の数
        let boxes = cnt.get_range(a.saturating_sub(D)..a) + 1;
        let balls = cnt[a];

        ans *= cmb.comb_with_rep(balls, boxes);
    }

    println!("{ans}");
}

#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::operation::Add, data_structure::segment_tree::SegmentTree,
};
use proconio::input;

/// 観察:
/// - 単調性 → なし
/// - { A: +1, B: -1, C: 0 } とすると，和が正である連続区間の数え上げになる
/// - 和が n になるような prefix の個数を持っておけば良さそう
fn main() {
    input! {
        N: usize,
        S: String,
    }

    // 0 の位置
    let mut O = 1001001;
    let mut dp = SegmentTree::<Add<usize>>::new(O * 2);

    let mut ans = 0_usize;

    for (i, c) in S.chars().enumerate() {
        match c {
            'A' => {
                // 全体が +1 される
                O -= 1;
                // 和が 1 であるような prefix が 1 増える
                *dp.get_mut(O + 1).unwrap() += 1;
            }
            'B' => {
                // 全体が -1 される
                O += 1;
                // 和が -1 であるような prefix が 1 増える
                *dp.get_mut(O - 1).unwrap() += 1;
            }
            _ => {
                // 和が 0 であるような prefix が 1 増える
                *dp.get_mut(O).unwrap() += 1;
            }
        }

        // 1 以上の区間の和
        ans += dp.get_range(O + 1..);
    }

    println!("{ans}");
}

#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::{
        actedmonoid::examples::{ArithAct, ArithSum, ArithVal},
        operation::Max,
    },
    data_structure::{lazy_segment_tree::LazySegmentTree, segment_tree::SegmentTree},
    debug,
};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [i64; N]
    }

    let seg = SegmentTree::<Max<(i64, usize)>>::from_vec(A.iter().copied().zip(0..).collect());

    // いもす法
    let mut ans =
        LazySegmentTree::<ArithSum<i64>>::from_vec((0..=N + 2).map(ArithVal::new).collect());

    for (i, &a) in A.iter().enumerate() {
        let (_, l) = seg.min_left(i, |v| v <= (a, i));
        let (_, r) = seg.max_right(i, |v| v <= (a, i));

        debug!(a, l, r, &A[l..r]);

        let llen = i - l;
        let rlen = r - i - 1;

        let x = llen.min(rlen);
        let y = llen.max(rlen);

        debug!(x, y, a);

        // 長さ 1 <= i <= x+1 の区間は i 個含まれる（それぞれ a ）
        ans.apply(1..=x + 1, ArithAct::new(a, 0));

        // 長さ x+1 < i <= y+1 の区間は x+1 個含まれる
        ans.apply(x + 2..=y + 1, ArithAct::new(0, a * (x as i64 + 1)));

        // 長さ y+1 < i <= x+y+1 の区間は (x+y+1)-i+1 = x+y-i 個含まれる
        ans.apply(y + 2..=x + y + 1, ArithAct::new(-a, a * (x + y + 2) as i64));
    }

    for i in 1..=N {
        println!("{}", ans.get_at(i).val_sum);
    }
}

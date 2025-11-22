#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::{monoid::Monoid, ordered_monoid::OrderedMonoid},
    data_structure::segment_tree::SegmentTree,
    debug,
    utils::{coordinate_compression::Compression, show_binary_tree::ShowBinaryTree},
};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
        A: [usize; N]
    }

    let Ai: Vec<_> = A.iter().enumerate().map(|(i, &a)| (a, i)).collect();

    // 座標圧縮
    let comp = Compression::new(&Ai);

    debug!(comp);

    // セグ木
    let mut seg = SegmentTree::<CntSum>::new(N);

    // ウィンドウの初期化
    for i in 0..M {
        let idx = comp.idx(&Ai[i]).unwrap();
        *seg.get_mut(idx).unwrap() = (1, A[i]);
    }

    seg.print_as_binary_tree();

    // 上位k個を取得
    let ((_, kth_sum), _) = seg.max_right(0, |x| x.0 <= K);
    println!("{kth_sum}");

    // ウィンドウ
    for (l, r) in (0..).zip(M..N) {
        // 削除
        let lidx = comp.idx(&Ai[l]).unwrap();
        *seg.get_mut(lidx).unwrap() = (0, 0);
        // 追加
        let ridx = comp.idx(&Ai[r]).unwrap();
        *seg.get_mut(ridx).unwrap() = (1, A[r]);

        seg.print_as_binary_tree();

        // 上位k個を取得
        let ((_, kth_sum), _) = seg.max_right(0, |x| x.0 <= K);
        println!("{kth_sum}");
    }
}

struct CntSum;

impl Monoid for CntSum {
    /// (count, sum)
    type Val = (usize, usize);
    fn id() -> Self::Val {
        (0, 0)
    }
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        (left.0 + right.0, left.1 + right.1)
    }
}

impl OrderedMonoid for CntSum {
    fn le(left: &Self::Val, right: &Self::Val) -> bool {
        left.0 <= right.0
    }
    fn lt(left: &Self::Val, right: &Self::Val) -> bool {
        left.0 < right.0
    }
}

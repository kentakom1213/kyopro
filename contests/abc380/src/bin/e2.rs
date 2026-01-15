#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::extmonoid::examples::UpdateMinMax,
    data_structure::lazy_segment_tree::LazySegmentTree, debug,
    utils::show_binary_tree::ShowBinaryTree,
};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
    }

    let mut cnt = vec![1; N + 1];
    let init = (1..=N).map(|v| (v, v)).collect::<Vec<_>>();
    let mut seg = LazySegmentTree::<UpdateMinMax<usize>>::from_vec(init);

    for _ in 0..Q {
        input! { t: usize }

        match t {
            1 => {
                input! {
                    x: Usize1,
                    c: usize,
                }
                seg.print_as_binary_tree();

                let v = seg.get(x..=x);
                let (_, l) = seg.min_left(x + 1, |(min, max)| v.0 <= min && max <= v.0);
                let (_, r) = seg.max_right(x, |(min, max)| v.0 <= min && max <= v.0);
                cnt[v.0] -= r - l;
                cnt[c] += r - l;

                debug!(x, v.0, l, r, c);
                seg.apply(l..r, (c, c));

                seg.print_as_binary_tree();
                debug!(cnt);
            }
            _ => {
                input! {
                    c: usize
                }
                println!("{}", cnt[c]);
            }
        }
    }
}

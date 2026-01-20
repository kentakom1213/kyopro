#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::actedmonoid::examples::UpdateMax,
    data_structure::dynamic_segment_tree_2d::DynamicSegmentTree2D,
};
use proconio::input;

fn main() {
    input! {
        N: usize,
        mut HWD: [(usize, usize, usize); N]
    }

    let mut seg =
        DynamicSegmentTree2D::<usize, UpdateMax<usize>>::new((0, 1001001001), (0, 1001001001));

    for &(h, w, d) in &HWD {
        seg.apply(h..=h, w..=w, d);
        seg.apply(h..=h, d..=d, w);
        seg.apply(w..=w, h..=h, d);
        seg.apply(w..=w, d..=d, h);
        seg.apply(d..=d, h..=h, w);
        seg.apply(d..=d, w..=w, h);
    }

    // 判定
    for &(h, w, d) in &HWD {
        let max_d = seg.get_range(h + 1.., w + 1..);

        if max_d > d {
            println!("Yes");
            return;
        }
    }

    println!("No");
}

#![allow(non_snake_case)]

use std::collections::VecDeque;

use cp_library_rs::{
    algebraic_structure::operation::Add, data_structure::segment_tree::SegmentTree, debug,
};
use proconio::input;

fn main() {
    input! {
        Q: usize,
    }

    let mut geta = SegmentTree::<Add<usize>>::new(Q + 3);
    let mut plants = VecDeque::new();

    for q in 0..Q {
        input!(t: usize);

        match t {
            1 => {
                plants.push_back(q);
            }
            2 => {
                input!(T: usize);
                *geta.get_mut(q).unwrap() += T;
            }
            3 => {
                input!(H: usize);
                let mut ans = 0;
                while let Some(&start) = plants.front() {
                    let wait = geta.get_range(start..);
                    if wait >= H {
                        plants.pop_front();
                        ans += 1;
                    } else {
                        break;
                    }
                }
                println!("{ans}");
            }
            _ => unreachable!(),
        }

        debug!(plants);
        debug!(geta);
    }
}

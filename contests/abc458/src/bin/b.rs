#![allow(non_snake_case)]

use cp_library_rs::utils::{boolutil::BoolUtil, grid::Grid};
use proconio::input;

fn main() {
    input! {
        H: usize,
        W: usize,
    }

    for r in 0..H {
        for c in 0..W {
            let ans = (r, c).get_adj_4((0, H), (0, W)).len();

            print!("{ans}{}", (c == W - 1).endl());
        }
    }
}

#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        L: usize,
        R: usize,
        XY: [(usize, usize); N]
    }

    let ans = XY.iter().filter(|&&(l, r)| l <= L && R <= r).count();

    println!("{ans}");
}

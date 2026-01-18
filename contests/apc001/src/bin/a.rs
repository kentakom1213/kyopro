#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        X: usize,
        Y: usize,
    }

    if X % Y == 0 {
        println!("-1");
        return;
    }

    println!("{X}");
}

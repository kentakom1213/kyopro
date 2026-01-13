#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
    }

    println!("{}", 2_usize.pow(N as u32) - 2 * N);
}

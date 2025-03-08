#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize
    }

    let x = (100 * N + 107) / 108;

    let res = 108 * x / 100;

    if res == N {
        println!("{x}");
    } else {
        println!(":(");
    }
}

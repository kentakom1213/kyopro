#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        A: [[usize]; N],
    }

    let res = (1..=M).filter(|i| A.iter().all(|a| a.contains(i))).count();

    println!("{res}");
}

#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [usize; N],
        K: usize
    }

    let ans = A.iter().filter(|&&a| K <= a).count();

    println!("{ans}");
}

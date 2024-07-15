#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    let g = A.iter().sum::<usize>() / N;

    let ans = A.iter().map(|&x| x.min(g)).sum::<usize>();

    println!("{ans}");
}

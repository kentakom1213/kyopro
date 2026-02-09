#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        K: usize,
        mut D: [usize; N]
    }

    D.sort_unstable();

    println!("{}", D.iter().take(N - K).sum::<usize>());
}

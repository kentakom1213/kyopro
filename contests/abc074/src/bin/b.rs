#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        K: usize,
        X: [usize; N]
    }

    let mut ans = 0;

    for &x in &X {
        ans += x.min(K - x) * 2;
    }

    println!("{ans}");
}

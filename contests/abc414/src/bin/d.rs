#![allow(non_snake_case)]

use cp_library_rs::debug;
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        mut X: [usize; N]
    }

    if M >= N {
        println!("0");
        return;
    }

    X.sort();

    // 差分
    let mut delta: Vec<_> = (1..N).map(|i| X[i] - X[i - 1]).collect();

    delta.sort();

    debug!(delta);

    let ans: usize = delta[..N - M].iter().sum();

    println!("{ans}");
}

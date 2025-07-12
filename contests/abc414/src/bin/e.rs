#![allow(non_snake_case)]

use cp_library_rs::{debug, number_theory::modint::M998, utils::num_traits::Zero};
use proconio::input;

fn main() {
    input! {
        N: usize,
    }

    let mut ans = M998::new(N) * (N + 1) / 2;

    // Σ_{b = 1}^N ⌊N / b⌋
    let mut i = 1;

    while i <= N {
        let q = N / i;
        let ni = N / q;
        ans -= q * (ni - i + 1);
        i = ni + 1;
    }

    println!("{ans}");
}

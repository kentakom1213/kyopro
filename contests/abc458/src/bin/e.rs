#![allow(non_snake_case)]

use cp_library_rs::number_theory::{comb::Comb, modint::M998};
use num::Zero;
use proconio::input;

fn main() {
    input! {
        X: usize,
        Y: usize,
        Z: usize
    }
    let N = X + Y + Z;

    let cmb = Comb::<M998>::new(N + 10);

    let mut ans = M998::zero();

    for k in 1..=Y {
        let choose_k = cmb.comb(Y + 1, k);
        let choose_1 = cmb.comb(X - 1, k - 1);
        let choose_3 = cmb.comb(Y + Z - k, Y - k);

        ans += choose_k * choose_1 * choose_3;
    }

    println!("{ans}");
}

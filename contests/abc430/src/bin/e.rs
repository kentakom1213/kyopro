#![allow(non_snake_case)]

use cp_library_rs::{
    debug, number_theory::modint_for_rollinghash::Modint, string::rolling_hash::RollingHash,
};
use proconio::input;

fn main() {
    input! {
        T: usize
    }

    for _ in 0..T {
        solve()
    }
}

fn solve() {
    input! {
        S: String,
        T: String
    }

    let N = S.len();

    let base = Modint::generate_base();
    let rh1 = RollingHash::from_str(&S, base);
    let rh2 = RollingHash::from_str(&T, base);

    // S の先頭をずらしていく
    for i in 0..N {
        let rot = rh1.hash(i..).chain(&rh1.hash(..i));

        debug!(rot);

        if rot == rh2.hash(..) {
            println!("{i}");
            return;
        }
    }

    println!("-1");
}

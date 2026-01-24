#![allow(non_snake_case)]

use cp_library_rs::{debug, number_theory::powmod::powmod};
use proconio::input;
use rustc_hash::FxHashMap;

const D: usize = 11;

fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize; N]
    }

    let mut cnt = vec![FxHashMap::default(); D];
    let mut ans = 0usize;

    for &a in &A {
        for d in 0..D {
            let m = (a % M) * powmod(10, d, M) % M;
            *cnt[d].entry(m).or_default() += 1;
        }
    }
    debug!(cnt);

    for &a in &A {
        let len_a = a.to_string().len();
        let rem = (M - a % M) % M;
        let tmp = cnt[len_a].get(&rem).unwrap_or(&0);
        debug!(a, len_a, rem, tmp);
        ans += tmp;
    }

    println!("{ans}");
}

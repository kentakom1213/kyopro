#![allow(non_snake_case)]

use proconio::input;
use rustc_hash::FxHashSet;

fn main() {
    input! {
        S: String
    }

    let mut set = FxHashSet::default();

    for l in 0..S.len() {
        for r in l..S.len() {
            set.insert(&S[l..=r]);
        }
    }

    println!("{}", set.len());
}

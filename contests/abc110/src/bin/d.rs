#![allow(non_snake_case)]

use cp_library_rs::{
    number_theory::{comb::Comb, factorize::factorize_pairs, modint::M107},
    utils::{consts::MOD107, num_traits::One},
};
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
    }

    let cmb = Comb::<MOD107>::new(202020);
    let mut res = M107::one();

    for (_, p) in factorize_pairs(M) {
        // p個の素因数をN個の箱に分ける組合せ
        res *= cmb.comb_with_rep(p, N);
    }

    println!("{res}");
}

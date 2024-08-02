#![allow(non_snake_case)]

use cp_library_rs::{debug, number_theory::modint::M998, utils::num_traits::One};
use proconio::input;

fn main() {
    input! {
        N: usize,
        K: usize,
        L: usize,
    }

    // Sの長さKのどの部分列も互いに異なる
    // <=> Sの任意の2つの同じ文字の組の間には，N-K個以上の別の文字が入っている

    let mut ans = M998::one();

    for i in 1..=N {
        // 直前N-K文字以内にでてきた文字は選べない
        if i <= N - K {
            ans *= (L + 1).saturating_sub(i);
        } else {
            ans *= (L + K).saturating_sub(N);
        }
    }

    println!("{ans}");
}

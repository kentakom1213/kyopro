#![allow(non_snake_case)]

use cp_library_rs::{
    number_theory::{
        modint::{Fp, M107},
        modint_comb::Comb,
    },
    utils::num_traits::Zero,
};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        K: usize,
        S: Chars
    }

    let N = S.len();

    let cmb = Comb::new(2001001);

    // K個を埋め込んだ後の文字列
    // SK以後の文字列の長さを全探索する
    let mut ans = M107::zero();

    for i in 0..=K {
        let a = 25.pow(K - i);
        let b = 26.pow(i);
        // 位置の選び方
        let c = cmb.comb(N + K - i - 1, N - 1);

        ans += a * b * c;
    }

    println!("{ans}");
}

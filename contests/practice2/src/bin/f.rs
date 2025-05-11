#![allow(non_snake_case)]

use cp_library_rs::{
    convolution::ntt::FFT,
    number_theory::modint::{Modint, M998},
    utils::num_traits::Zero,
};
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        mut A: [M998; N],
        mut B: [M998; M]
    }

    A.extend((0..M).map(|_| Modint::zero()));
    B.extend((0..N).map(|_| Modint::zero()));

    let fa = FFT::fft(&A).unwrap();
    let fb = FFT::fft(&B).unwrap();

    let conv: Vec<_> = fa
        .into_iter()
        .zip(fb.into_iter())
        .map(|(a, b)| a * b)
        .collect();

    // 復元
    let C = FFT::ifft(&conv).unwrap();

    println!(
        "{}",
        C.iter()
            .chain(std::iter::repeat(&Modint::zero()))
            .take(N + M - 1)
            .join(" ")
    );
}

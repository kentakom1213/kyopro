#![allow(non_snake_case)]

use cp_library_rs::{debug, utils::run_length::RunLength};
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    let a_max = *A.iter().max().unwrap();

    let mut B = vec![false; a_max + 1];

    for (&a, n) in A.iter().sorted().run_length_encode() {
        if n == 1 {
            B[a] = true;
        }
    }

    for &a in &A {
        for k in 2.. {
            if a * k > a_max {
                break;
            }
            B[a * k] = false;
        }
    }

    debug!(B);

    let ans = B.iter().filter(|&&x| x).count();

    println!("{ans}");
}

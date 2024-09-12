#![allow(non_snake_case)]

use cp_library_rs::utils::iterutil::IterUtil;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut S: Chars,
        T: Chars
    }

    let N = S.len();

    let diff = S.iter().zip(&T).filter(|&(c, d)| c != d).count();
    println!("{diff}");

    for i in 0..N {
        if S[i] > T[i] {
            S[i] = T[i];
            println!("{}", S.iter().join(""));
        }
    }

    for i in (0..N).rev() {
        if S[i] < T[i] {
            S[i] = T[i];
            println!("{}", S.iter().join(""));
        }
    }
}

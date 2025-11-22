#![allow(non_snake_case)]

use cp_library_rs::utils::yesno::YesNo;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        S: [String; N],
        X: Usize1,
        Y: String
    }

    println!("{}", (S[X] == Y).yesno());
}

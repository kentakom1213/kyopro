#![allow(non_snake_case)]

use cp_library_rs::utils::yesno::YesNo;
use proconio::input;

fn main() {
    input! {
        N: usize,
        T: usize,
        A: usize,
    }

    println!("{}", (N / 2 < T || N / 2 < A).yesno());
}

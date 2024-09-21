#![allow(non_snake_case)]

use cp_library_rs::utils::iterutil::IterUtil;
use proconio::input;

fn main() {
    input! {
        S: String
    }

    println!("{}", S.chars().filter(|&c| c != '.').join(""));
}

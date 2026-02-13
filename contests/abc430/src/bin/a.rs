#![allow(non_snake_case)]

use cp_library_rs::utils::yesno::YesNo;
use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize
    }

    let isok = if c >= a { d >= b } else { true };

    println!("{}", (!isok).yesno());
}

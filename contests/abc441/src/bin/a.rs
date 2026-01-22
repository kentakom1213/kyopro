#![allow(non_snake_case)]

use cp_library_rs::utils::yesno::YesNo;
use proconio::input;

fn main() {
    input! {
        p: usize,
        q: usize,
        x: usize,
        y: usize
    }

    let isok = p <= x && x < p + 100 && q <= y && y < q + 100;

    println!("{}", isok.yesno());
}

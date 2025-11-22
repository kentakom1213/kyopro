#![allow(non_snake_case)]

use cp_library_rs::utils::yesno::YesNo;
use proconio::input;

fn main() {
    input! {
        x: isize,
        y: isize,
        z: isize,
    }

    let isok = (x >= z * y) && ((x - z * y) % (z - 1) == 0);

    println!("{}", isok.yesno());
}

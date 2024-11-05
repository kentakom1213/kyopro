#![allow(non_snake_case)]

use cp_library_rs::utils::yesno::YesNo;
use proconio::input;

fn main() {
    input! {
        S: String
    }

    let isok = S.ends_with("san");

    println!("{}", isok.yesno());
}

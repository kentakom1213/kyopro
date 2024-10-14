#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        R: isize,
    }

    let res = if R <= 99 {
        100 - R
    } else if R <= 199 {
        200 - R
    } else {
        300 - R
    };

    println!("{res}");
}

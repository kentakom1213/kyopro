#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        Y: usize,
    }

    let ans = 
    if Y % 4 != 0 {
        365
    } else if Y % 100 != 0 {
        366
    } else if Y % 400 != 0 {
        365
    } else {
        366
    };

    println!("{ans}");
}


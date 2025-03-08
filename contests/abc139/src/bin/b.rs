#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        A: usize,
        B: usize,
    }

    let ans = (B - 1 + (A - 1) - 1) / (A - 1);
    
    println!("{ans}");
}


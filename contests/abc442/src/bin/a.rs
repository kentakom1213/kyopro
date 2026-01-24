#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        S: String
    }

    let ans = S.chars().filter(|&c| c == 'i' || c == 'j').count();

    println!("{ans}");
}

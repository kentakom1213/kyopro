#![allow(non_snake_case)]

use cp_library_rs::utils::consts::ASCII_LOWERCASE_ARR;
use proconio::input;

fn main() {
    input! {
        N: usize
    }

    println!("{}", encode(N));
}

/// excel進法にエンコードする
fn encode(mut n: usize) -> String {
    let mut res = String::new();

    while n > 0 {
        let c = ASCII_LOWERCASE_ARR[(n + 25) % 26];
        n /= 26;
        // 繰り上がりの調整
        if c == 'z' {
            n -= 1;
        }
        res.push(c);
    }

    res = res.chars().rev().collect();

    res
}

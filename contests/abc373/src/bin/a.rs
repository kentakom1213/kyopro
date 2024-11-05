#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        S: [String; 12]
    }

    let mut ans = 0;

    for (i, s) in S.iter().enumerate() {
        if s.len() == i + 1 {
            ans += 1;
        }
    }

    println!("{ans}");
}


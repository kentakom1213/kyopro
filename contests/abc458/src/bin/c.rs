#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        S: String
    }
    let N = S.len();

    let mut ans = 0;

    for (i, c) in S.chars().enumerate() {
        let l = i;
        let r = N - i - 1;

        if c == 'C' {
            ans += l.min(r) + 1;
        }
    }

    println!("{ans}");
}

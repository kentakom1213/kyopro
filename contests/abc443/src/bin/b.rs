#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        K: usize,
    }

    let mut sum = 0;
    let mut ans = 0;

    for n in N.. {
        sum += n;
        if sum >= K {
            break;
        }
        ans += 1;
    }

    println!("{ans}");
}

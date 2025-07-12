#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
    }

    let mut ans = 0;
    let mut i = 1;

    while i <= N {
        let k = N / i;
        // kを維持できる最大のi
        let max_i = N / k;

        ans += k * (max_i - i + 1);

        i = max_i + 1;
    }

    println!("{ans}");
}

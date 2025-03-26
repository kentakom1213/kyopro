#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
    }

    let h = (N - 1) / 2;

    let res = format!(
        "{}{}{}",
        "-".repeat(h),
        if N % 2 == 0 { "==" } else { "=" },
        "-".repeat(h)
    );

    println!("{res}");
}

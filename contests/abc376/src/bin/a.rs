#![allow(non_snake_case)]

use cp_library_rs::utils::consts::IINF;
use proconio::input;

fn main() {
    input! {
        N: usize,
        C: isize,
        T: [isize; N]
    }

    let mut ans = 0;
    let mut last = -IINF;

    for &t in &T {
        if t - last >= C {
            ans += 1;
            last = t;
        }
    }

    println!("{ans}");
}

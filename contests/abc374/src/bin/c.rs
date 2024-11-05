#![allow(non_snake_case)]

use cp_library_rs::{chmin, utils::consts::INF};
use proconio::input;

fn main() {
    input! {
        N: usize,
        K: [usize; N]
    }

    let mut ans = INF;

    for i in 0..1 << N {
        let mut a = 0;
        let mut b = 0;
        for j in 0..N {
            if i >> j & 1 == 1 {
                a += K[j];
            } else {
                b += K[j];
            }
        }
        chmin!(ans, a.max(b));
    }

    println!("{ans}");
}

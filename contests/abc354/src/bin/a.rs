#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        H: usize,
    }

    let mut d = 1_usize;
    let mut h = 0;

    for i in 0..40 {
        if h > H {
            println!("{i}");
            return;
        }
        h += d;
        d *= 2;
    }
}

const _INF: usize = 1001001001001001001;

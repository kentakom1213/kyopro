#![allow(non_snake_case)]

use cp_library_rs::debug;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        TV: [(usize, usize); N]
    }

    let mut d = vec![0; MAX + 1];

    for &(t, v) in &TV {
        d[t] += v;
    }

    for i in 1..=MAX {
        d[i] += d[i - 1];

        if d[i] > 0 {
            d[i] -= 1;
        }
    }

    debug!(d);

    let t = TV[N - 1].0;
    println!("{}", d[t] + 1);
}

const MAX: usize = 200;

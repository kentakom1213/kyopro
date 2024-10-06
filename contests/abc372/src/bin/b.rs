#![allow(non_snake_case)]

use cp_library_rs::debug;
use proconio::input;

fn main() {
    input! {
        M: usize,
    }

    let mut m = M;
    let mut res = vec![];
    while m > 0 {
        res.push(m % 3);
        m /= 3;
    }

    debug!(res);

    let N = res.iter().sum::<usize>();
    println!("{}", N);

    for (i, &x) in res.iter().enumerate() {
        for _ in 0..x {
            print!("{} ", i);
        }
    }
    println!();
}

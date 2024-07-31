#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [usize; 7 * N]
    }

    for i in 0..N {
        let mut s = 0;
        for j in 0..7 {
            s += A[i * 7 + j];
        }
        print!("{s} ");
    }
    println!();
}

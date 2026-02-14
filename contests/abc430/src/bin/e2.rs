#![allow(non_snake_case)]

use cp_library_rs::{debug, string::z::z_algorithm};
use proconio::input;

fn main() {
    input! {
        T: usize,
    }
    'outer: for _ in 0..T {
        input! {
            A: String,
            B: String,
        }

        let N = A.len();

        let mut S = B;
        S += &A;
        S += &A;

        let z = z_algorithm(S.as_bytes());

        debug!(z);

        for i in 0..N {
            if z[N + i] >= N {
                println!("{i}");
                continue 'outer;
            }
        }

        println!("-1");
    }
}

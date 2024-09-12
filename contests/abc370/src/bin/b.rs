#![allow(non_snake_case)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
    }

    let mut A = vec![];

    for i in 1..=N {
        input! {
            a: [Usize1; i]
        }
        A.push(a);
    }

    let mut elem = 0;

    for j in 0..N {
        if elem >= j {
            elem = A[elem][j];
        } else {
            elem = A[j][elem];
        }
    }

    println!("{}", elem + 1);
}

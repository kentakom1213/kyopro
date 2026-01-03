#![allow(non_snake_case)]

use proconio::input;

const NEG1: usize = 1_usize.wrapping_neg();

fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    for i in 0..N {
        let mut ans = NEG1;
        for j in 0..i {
            if A[j] > A[i] {
                ans = j + 1;
            }
        }

        println!("{}", ans as isize);
    }
}


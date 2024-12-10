#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        C: isize,
        B: [isize; M],
        A: [[isize; M]; N],
    }

    let mut ans = 0;

    for i in 0..N {
        let mut tmp = 0;

        for j in 0..M {
            tmp += A[i][j] * B[j];
        }

        if tmp + C > 0 {
            ans += 1;
        }
    }

    println!("{ans}");
}

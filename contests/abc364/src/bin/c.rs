#![allow(non_snake_case)]

use cp_library_rs::chmin;
use proconio::input;

fn main() {
    input! {
        N: usize,
        X: usize,
        Y: usize,
        mut A: [usize; N],
        mut B: [usize; N],
    }

    let mut ans = N;

    // Aで見る
    A.sort();
    A.reverse();
    let mut tmp = 0;
    for (i, &a) in (1..).zip(&A) {
        tmp += a;
        if tmp > X {
            chmin! {
                ans,
                i
            };
        }
    }

    // Bでみる
    B.sort();
    B.reverse();
    tmp = 0;
    for (i, &b) in (1..).zip(&B) {
        tmp += b;
        if tmp > Y {
            chmin! {
                ans,
                i
            };
        }
    }

    println!("{ans}");
}

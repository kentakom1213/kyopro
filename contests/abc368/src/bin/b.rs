#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        mut A: [usize; N]
    }

    // 愚直に試す
    let mut ans = 0;

    A.sort();
    A.reverse();

    while A[1] > 0 {
        ans += 1;

        A[0] -= 1;
        A[1] -= 1;

        A.sort();
        A.reverse();
    }

    println!("{ans}");
}

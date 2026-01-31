#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        T: usize,
        A: [usize; N]
    }

    let mut ans = 0;
    let mut nxt = 0;

    for &a in &A {
        if a >= nxt {
            ans += a - nxt;
            nxt = a + 100;
        }
    }

    ans += T.saturating_sub(nxt);

    println!("{ans}");
}

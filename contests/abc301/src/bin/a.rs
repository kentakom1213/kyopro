#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        S: String
    }

    let T = S.chars().filter(|&c| c == 'T').count();
    let A = N - T;

    if T > A {
        println!("T");
        return;
    } else if T < A {
        println!("A");
        return;
    }

    let mut t = 0;
    let mut a = 0;

    for c in S.chars() {
        if c == 'T' {
            t += 1;
        } else {
            a += 1;
        }

        if t == T {
            println!("T");
            return;
        } else if a == A {
            println!("A");
            return;
        }
    }
}

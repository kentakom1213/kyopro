#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        S: [String; N]
    }

    let mut issw = false;
    let mut isng = false;

    for s in S {
        if isng {
            println!("No");
            return;
        }

        if s == "sweet" {
            if issw {
                isng = true;
            } else {
                issw = true;
            }
        } else {
            issw = false;
        }
    }

    println!("Yes");
}

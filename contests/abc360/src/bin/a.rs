#![allow(non_snake_case)]

use cp_library_rs::yesno::YesNo;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        S: Chars
    }

    let mut r = 0;
    let mut m = 0;
    for i in 0..3 {
        if S[i] == 'R' {
            r = i;
        }
        if S[i] == 'M' {
            m = i;
        }
    }

    println!("{}", (r < m).yesno());
}

#![allow(non_snake_case)]

use cp_library_rs::{debug, utils::yesno::YesNo};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        A: usize,
        B: usize,
        S: String
    }

    let mut total = 0;
    let mut overseas = 0;

    for c in S.chars() {
        let is_pass = match c {
            'a' => {
                if total < A + B {
                    total += 1;
                    true
                } else {
                    false
                }
            }
            'b' => {
                if total < A + B && overseas < B {
                    total += 1;
                    overseas += 1;
                    true
                } else {
                    false
                }
            }
            'c' => false,
            _ => unreachable!(),
        };

        debug!(total, overseas, is_pass);

        println!("{}", is_pass.yesno());
    }
}

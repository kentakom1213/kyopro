#![allow(non_snake_case)]

use cp_library_rs::utils::yesno::YesNo;
use proconio::input;

fn main() {
    input! {
        Q: usize,
        A: [u8; Q]
    }

    let mut sound = 0;
    let mut isplay = false;

    for &a in &A {
        match a {
            1 => {
                sound += 1;
            }
            2 => {
                if sound > 0 {
                    sound -= 1;
                }
            }
            3 => {
                isplay ^= true;
            }
            _ => unreachable!(),
        }

        let ans = sound >= 3 && isplay;

        println!("{}", ans.yesno());
    }
}

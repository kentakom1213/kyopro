#![allow(non_snake_case)]

use cp_library_rs::utils::{run_length::RunLength, yesno::YesNo};
use proconio::input;

fn main() {
    input! {
        S: String
    }

    if S.len() % 2 == 1 {
        println!("No");
        return;
    }

    let cnt = S.chars().fold([0; 26], |mut cnt, x| {
        let i = x as usize - 'a' as usize;
        cnt[i] += 1;
        cnt
    });

    if !cnt.iter().all(|&x| x == 0 || x == 2) {
        println!("No");
        return;
    }

    let isok = S
        .chars()
        .run_length_encode()
        .into_iter()
        .all(|(_, x)| x == 2);

    println!("{}", isok.yesno());
}

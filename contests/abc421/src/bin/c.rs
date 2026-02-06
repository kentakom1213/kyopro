#![allow(non_snake_case)]

use std::collections::VecDeque;

use cp_library_rs::{debug, utils::run_length::RunLength};
use proconio::input;

fn main() {
    input! {
        N: usize,
        S: String,
    }

    let mut acnt = 0;
    let mut ans1 = 0;
    let mut ans2 = 0;

    for (i, c) in S.chars().enumerate() {
        match c {
            'A' => {
                ans1 += i.abs_diff(acnt * 2);
                ans2 += i.abs_diff(acnt * 2 + 1);
                acnt += 1;
            }
            _ => {}
        }
    }

    println!("{}", ans1.min(ans2));
}

#![allow(non_snake_case)]

use cp_library_rs::{
    debug,
    utils::palindrome::{generate_palindrome_number, is_palindrome},
};
use proconio::input;

fn main() {
    input! {
        A: usize,
        N: usize
    }

    let ans: usize = generate_palindrome_number(N)
        .filter(|&n| is_palindrome(n, A))
        .sum();

    println!("{ans}");
}

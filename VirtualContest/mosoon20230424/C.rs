// https://atcoder.jp/contests/abc232/tasks/abc232_b

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// ## ord
/// `A`を`0`とするascii文字(`A~Za~z`)のインデックスを返す
fn ord(c: char) -> isize {
    let a = 'A' as u32;
    let c = c as u32;
    (c - a) as isize
}

const ASCII_LOWERCASE: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

// main
fn main() {
    input! {
        S: String,
        T: String,
    }

    let mut itr = S.chars().zip(T.chars()).map(|(s, t)| (26 + ord(s) - ord(t)) % 26);

    let top = itr.next().unwrap();

    if itr.all(|x| x == top) {
        println!("Yes");
    } else {
        println!("No");
    }
}
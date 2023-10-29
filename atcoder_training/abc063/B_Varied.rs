//                B - Varied               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc063/tasks/abc063_b
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

// imports
use std::collections::HashSet;
use std::cmp::Reverse;

// solve
fn main() {
    let mut S = String::new();
    std::io::stdin().read_line(&mut S).ok();
    let S: Vec<char> = S.trim().chars().collect();

    let is_ok = S.iter().cloned().collect::<HashSet<char>>().len() == S.len();
    println!("{}", if is_ok { "yes" } else { "no" });
}


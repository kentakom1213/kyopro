#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        S: String
    }

    let x = S.split('/').collect_vec();

    if let &[a, b] = &x[..] {
        let isok = a.len() == b.len() && a.chars().all(|x| x == '1') && b.chars().all(|x| x == '2');
        if isok {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        println!("No");
    }
}

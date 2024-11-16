#![allow(non_snake_case)]

use cp_library_rs::debug;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        S: String
    }

    let sp = S
        .split('|')
        .filter(|&s| !s.is_empty())
        .map(|x| x.len())
        .collect_vec();

    debug!(sp);

    println!("{}", sp.iter().join(" "));
}

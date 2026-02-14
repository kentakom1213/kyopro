#![allow(non_snake_case)]

use cp_library_rs::utils::boolutil::BoolUtil;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        S: Chars
    }

    let ans = S[0] == S[S.len() - 1];

    println!("{}", ans.yesno());
}

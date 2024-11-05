#![allow(non_snake_case)]

use cp_library_rs::debug;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        S: String
    }

    // インデックス
    let mut idx = [0; 26];

    for (i, c) in S.chars().enumerate() {
        let x = c as usize - 'A' as usize;
        idx[x] = i;
    }

    debug!(idx);

    let mut ans = 0;
    let mut cur = idx[0];

    for &x in &idx {
        ans += x.abs_diff(cur);
        cur = x;
    }

    println!("{ans}");
}

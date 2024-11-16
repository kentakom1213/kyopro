#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        S: Chars,
        Q: usize,
        K: [Usize1; Q],
    }

    let N = S.len();

    let mut ans = vec![];

    for &k in &K {
        // 何文字目か
        let r = k % N;
        let chr = S[r];

        // 大文字小文字の判定
        let q = k / N;
        let cnt = q.count_ones();

        if cnt % 2 == 0 {
            ans.push(chr);
        } else {
            ans.push(flip(chr));
        }
    }

    println!("{}", ans.iter().join(" "));
}

fn flip(c: char) -> char {
    if c.is_uppercase() {
        // 小文字に
        c.to_ascii_lowercase()
    } else {
        // 大文字に
        c.to_ascii_uppercase()
    }
}

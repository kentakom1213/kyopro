#![allow(non_snake_case)]

use cp_library_rs::{debug, utils::iterutil::IterUtil};
use permutohedron::LexicalPermutation;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
        K: usize,
        mut S: Chars
    }

    let has_k_palindrome = |s: &[char]| -> bool {
        // sに長さKの回文が存在するか
        let mut cnt = 0;
        for i in 0..N - K + 1 {
            let mut ok = true;
            for j in 0..K / 2 {
                if s[i + j] != s[i + K - j - 1] {
                    ok = false;
                    break;
                }
            }
            cnt += ok as usize;
        }
        cnt > 0
    };

    S.sort();
    let mut ans = 0;

    while {
        let has = !has_k_palindrome(&S);

        ans += has as usize;

        S.next_permutation()
    } {}

    println!("{ans}");
}

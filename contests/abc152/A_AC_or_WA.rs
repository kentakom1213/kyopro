//               A - AC or WA
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc152/tasks/abc152_a
// ----------------------------------------

use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
    }

    if N == M {
        println!("Yes");
    } else {
        println!("No");
    }
}

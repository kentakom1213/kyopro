//                A - Filter               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc294/tasks/abc294_a
// ----------------------------------------

use itertools::Itertools;

fn main() {
    proconio::input!{
        N: usize,
        A: [usize; N],
    }
    println!("{}", A.iter().filter(|&v| v % 2 == 0).join(" "));
}
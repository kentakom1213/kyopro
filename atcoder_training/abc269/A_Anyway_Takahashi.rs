//           A - Anyway Takahashi          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc269/tasks/abc269_a
// ----------------------------------------

use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
    }

    println!("{}", (a+b)*(c-d));
    println!("Takahashi");
}


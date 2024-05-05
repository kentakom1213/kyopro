//              B - Mix Juice              
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc171/tasks/abc171_b
// ----------------------------------------

use proconio::input;

// solve
fn main() {
    input! {
        n: usize,
        k: usize,
        mut p: [usize; n],
    }

    // ソート
    p.sort();

    let ans = p[0..k].iter().fold(0, |acc, x| acc + x);

    println!("{}", ans);
}

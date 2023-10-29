//            D - Two Sequences
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc091/tasks/arc092_b
// ----------------------------------------

use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [usize; N],
        B: [usize; N],
    }

    let mut ans = 0;
    for &a in &A {
        for &b in &B {
            ans ^= a + b;
        }
    }

    println!("{}", ans);
}

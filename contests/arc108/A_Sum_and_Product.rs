//           A - Sum and Product           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc108/tasks/arc108_a
// ----------------------------------------

use proconio::input;

fn main() {
    input! {
        S: usize,
        P: usize,
    }

    // 約数列挙
    for i in 1..=P {
        if i*i > P { break; }

        if P % i == 0 && i + P/i == S {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

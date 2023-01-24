//           B - Sandwich Number           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc281/tasks/abc281_b
// ----------------------------------------

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut is_ok = s.len() == 8;

    for (i, c) in s.chars().enumerate() {
        if i == 0 || i == 7 {
            is_ok &= c.is_ascii_uppercase();
        } else if i == 1 {
            is_ok &= c.is_digit(10) && c != '0';
        } else {
            is_ok &= c.is_digit(10);
        }
    }

    if is_ok {
        println!("Yes");
    } else {
        println!("No");
    }
}

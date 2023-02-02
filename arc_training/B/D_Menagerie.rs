//              D - Menagerie
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc055/tasks/arc069_b
// ----------------------------------------

#![allow(non_snake_case)]

use proconio::input;

/// ## 方針
/// - 最初に2匹が決まれば連鎖的に定まる性質を使う
fn main() {
    input! {
        N: usize,
        S: String,
    }

    // 最初の2匹がSSのとき
    let mut A = "SS".to_string();
    for c in S.chars() {
        match (c, &A[A.len()-2..]) {
            ('o', "SS") => {

            },
            ('o', "SW") => (),
            ('o', "WS") => (),
            ('o', "WW") => (),
            ('x', "SS") => (),
            ('x', "SW") => (),
            ('x', "WS") => (),
            ('x', "WW") => (),
            _ => unreachable!(),
        }
    }
}

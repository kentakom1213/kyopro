#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        mut S: String,
        mut T: String
    }

    let sl = S.len();
    let tl = T.len();

    let max = sl.max(tl);

    if sl < max {
        S += &" ".repeat(max - sl);
    }

    if tl < max {
        T += &" ".repeat(max - tl);
    }

    if S == T {
        println!("0");
        return;
    }

    // 差分を検出
    let mut diff = 0;
    for (s, t) in S.chars().zip(T.chars()) {
        diff += 1;
        if s != t {
            break;
        }
    }

    println!("{}", diff);
}

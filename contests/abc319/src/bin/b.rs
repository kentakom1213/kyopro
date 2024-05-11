#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize
    }

    let mut ans = String::new();

    for i in 0..=N {
        let mut isok = false;
        for j in 1..=9 {
            if N % j == 0 && i % (N / j) == 0 {
                ans += &j.to_string();
                isok = true;
                break;
            }
        }
        if !isok {
            ans.push('-');
        }
    }

    println!("{ans}");
}

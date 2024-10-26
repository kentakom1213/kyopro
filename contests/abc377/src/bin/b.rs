#![allow(non_snake_case)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        S: [Chars; 8]
    }

    let mut ans = 0;

    for x in 0..8 {
        for y in 0..8 {
            if S[x][y] == '#' {
                continue;
            }
            let mut isok = true;
            for i in 0..8 {
                isok &= S[i][y] == '.';
            }
            for j in 0..8 {
                isok &= S[x][j] == '.';
            }
            if isok {
                ans += 1;
            }
        }
    }

    println!("{ans}");
}

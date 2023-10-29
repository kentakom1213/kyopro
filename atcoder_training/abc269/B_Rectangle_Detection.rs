//         B - Rectangle Detection         
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc269/tasks/abc269_b
// ----------------------------------------

use proconio::input;

fn main() {
    input! {
        s: [String; 10],
    }

    let (mut a, mut b, mut c, mut d) = (0, 0, 0, 0);
    for i in 0..10 {
        for j in 0..10 {
            if &s[i][j..j+1] == "#" {
                if a == 0 {
                    a = i+1;
                }
                if c == 0 {
                    c = j+1;
                }
                b = i+1;
                d = j+1;
            }
        }
    }

    println!("{} {}", a, b);
    println!("{} {}", c, d);
}


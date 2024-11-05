#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        H: [isize; N]
    }

    let mut T = 0;

    for &h in &H {
        // まとめて処理
        let mut h = {
            T += h / 5 * 3;
            h % 5
        };
        // 愚直に処理
        while h > 0 {
            T += 1;
            if T % 3 == 0 {
                h -= 3;
            } else {
                h -= 1;
            }
        }
    }

    println!("{T}");
}

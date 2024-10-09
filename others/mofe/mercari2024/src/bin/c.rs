#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
       W: isize,
       H: isize,
       T: isize,
       sx: isize,
       sy: isize
    }

    // x,yは独立に考えることができる
    let mut x = (sx + T) % (2 * W);
    let mut y = (sy + T) % (2 * H);

    if x > W {
        x = (2 * W - x) % W;
    }

    if y > H {
        y = (2 * H - y) % H;
    }

    println!("{} {}", x, y);
}

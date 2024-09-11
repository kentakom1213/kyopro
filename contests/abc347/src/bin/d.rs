#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        C: usize,
    }

    let c = C.count_ones() as usize;

    let k2 = a + b - c;

    if k2 % 2 == 1 {
        println!("-1");
        return;
    }

    let k = k2 / 2;

    if c + k > 60 {
        println!("-1");
        return;
    }

    // 貪欲に構築
    let mut X: usize = 0;
    let mut Y: usize = 0;
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    for i in 0..60 {
        if C >> i & 1 == 1 {
            if x < a - k {
                X |= 1 << i;
                x += 1;
            } else if y < b - k {
                Y |= 1 << i;
                y += 1;
            }
        } else {
            if z < k {
                X |= 1 << i;
                Y |= 1 << i;
                z += 1;
            }
        }
    }

    eprintln!("{X:0>60b}");
    eprintln!("{Y:0>60b}");

    // assert_eq!(X.count_ones() as usize, a);
    // assert_eq!(Y.count_ones() as usize, b);
    // assert_eq!(X ^ Y, C);

    if X.count_ones() as usize != a || Y.count_ones() as usize != b || X ^ Y != C {
        println!("-1");
        return;
    }

    println!("{X} {Y}");
}

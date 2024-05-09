#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        L: usize,
        R: usize,
    }

    let mut a = vec![0; R + 1];

    for i in 2..=R {
        a[i] = (R / i - (L - 1) / i).pow(2);
    }

    for i in (2..=R).rev() {
        for j in 2.. {
            if i * j > R {
                break;
            }
            a[i] -= a[i * j];
        }
    }

    for i in L..=R {
        a[i] = a[i].wrapping_sub((R / i - (L - 1) / i) * 2 - 1);
    }

    let ans: usize = a[2..=R].iter().sum();

    println!("{ans}");
}

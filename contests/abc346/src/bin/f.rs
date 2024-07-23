#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        L: usize,
        S: [String; N]
    }

    let mut T = vec![0_u32; N];

    for i in 0..N {
        for c in S[i].chars() {
            T[i] |= 1 << ord(c);
        }
    }

    #[cfg(debug_assertions)]
    for i in 0..N {
        println!("{:0>26b}", T[i]);
    }
}

fn ord(c: char) -> usize {
    c as usize - 'a' as usize
}

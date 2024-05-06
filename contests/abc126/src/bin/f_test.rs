#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use permutohedron::LexicalPermutation;
use proconio::input;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}
macro_rules! debug2D {
    ( $array:expr ) => {{
        #![cfg(debug_assertions)]
        eprintln!("{}: ", stringify!($array));
        for row in &$array {
            eprintln!("{:?}", row);
        }
    }};
}

fn main() {
    input! {
        M: usize,
        K: usize,
    }

    let N: usize = 1 << M + 1;

    let mut A = (0..1 << M).fold(vec![0; N], |mut v, i| {
        v[2 * i] = i;
        v[2 * i + 1] = i;
        v
    });

    let isok = |a: &[usize]| -> bool {
        for i in 0..N {
            for j in i + 1..N {
                if a[i] != a[j] {
                    continue;
                }
                // a[i] == a[j]のとき
                let s = (i..=j).fold(0, |acc, x| acc ^ a[x]);

                if s != K {
                    return false;
                }
            }
        }
        true
    };

    loop {
        if isok(&A) {
            println!("OK: {:?}", A);
        }

        if !A.next_permutation() {
            break;
        }
    }
}

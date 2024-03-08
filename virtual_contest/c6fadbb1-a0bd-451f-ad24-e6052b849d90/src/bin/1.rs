#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        A: [isize; N],
    }

    // 交互に足したものが X1
    let X1 = A
        .iter()
        .enumerate()
        .map(|(i, &a)| if i % 2 == 0 { a } else { -a })
        .sum::<isize>();

    let mut Xs = vec![X1];

    for i in 1..N {
        let xi = A[i - 1] - Xs[i - 1] / 2;
        let Xi = 2 * xi;
        Xs.push(Xi);
    }

    println!("{}", Xs.iter().join(" "));
}

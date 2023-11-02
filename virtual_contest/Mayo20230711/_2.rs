// https://atcoder.jp/contests/abc079/tasks/abc079_c

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// main
fn main() {
    input! {
        X: String,
    }

    let abcd = X
        .chars()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect::<Vec<isize>>();

    if let [a, b, c, d] = abcd[..4] {
        if (a + b + c + d) == 7 {
            println!("{}+{}+{}+{}=7", a, b, c, d);
        } else if (a + b + c - d) == 7 {
            println!("{}+{}+{}-{}=7", a, b, c, d);
        } else if (a + b - c + d) == 7 {
            println!("{}+{}-{}+{}=7", a, b, c, d);
        } else if (a + b - c - d) == 7 {
            println!("{}+{}-{}-{}=7", a, b, c, d);
        } else if (a - b + c + d) == 7 {
            println!("{}-{}+{}+{}=7", a, b, c, d);
        } else if (a - b + c - d) == 7 {
            println!("{}-{}+{}-{}=7", a, b, c, d);
        } else if (a - b - c + d) == 7 {
            println!("{}-{}-{}+{}=7", a, b, c, d);
        } else if (a - b - c - d) == 7 {
            println!("{}-{}-{}-{}=7", a, b, c, d);
        }
    }
}

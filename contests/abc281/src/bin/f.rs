#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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

use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    let ans = get_max(30, A);

    println!("{ans}");
}

/// 上からi-1桁目までが0の数の集合Aの中の最小値
fn get_max(i: usize, mut A: Vec<usize>) -> usize {
    if cfg!(debug_assertions) {
        eprintln!(
            "{}{}",
            "  ".repeat(30 - i),
            A.iter().map(|v| format!("{:b}", v)).join(", ")
        );
    }

    // 数が1つのとき
    if A.len() == 0 {
        return 0;
    }
    let mut zeros = vec![];
    let mut ones = vec![];

    // i桁目の符号によって分割
    while let Some(a) = A.pop() {
        if a >> i & 1 == 1 {
            ones.push(a & !(1 << i));
        } else {
            zeros.push(a);
        }
    }

    // i桁目が0/1の両方を含むとき，1桁目は1になる
    match (!zeros.is_empty(), !ones.is_empty()) {
        (true, true) => {
            (1 << i)
                + if i > 0 {
                    get_max(i - 1, zeros).min(get_max(i - 1, ones))
                } else {
                    0
                }
        }
        (true, false) if i > 0 => get_max(i - 1, zeros),
        (false, true) if i > 0 => get_max(i - 1, ones),
        _ => 0,
    }
}
